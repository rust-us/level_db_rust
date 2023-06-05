use std::{intrinsics, ptr};
use std::alloc::{alloc, Layout};
use std::ops::Deref;

use crate::util::coding::EncodeData::{Buffer, Slices, Vector};
use crate::util::coding::MutEncodeData::{MutBuffer, MutSlices, MutVector};
use crate::util::Result;
use crate::util::slice::Slice;
use crate::util::status::LevelError;

/// 获取变长编码的长度 varint需要的字节数
///
/// # Arguments
///
/// * `value`: 待编码数据
///
/// returns: usize
///
/// # Examples
///
/// ```
/// use level_db_rust::util::coding::varint_length;
/// // length == 2
/// let length = varint_length(255);
/// ```
pub fn varint_length(mut value: u64) -> usize {
    let mut len = 1;
    // varint每7位编码一次, 所以相对于8位一个字节的原数据, 大数进行varint编码的总字节会多于原来的字节数
    // 当value右移之后 > 128, 说明下一位还有数据
    while value >= 128 {
        value >>= 7;
        len += 1;
    }
    len
}

/// 默认为小端bytes 当系统为小端时, 这个宏会生效, 小端系统居多
#[cfg(target_endian = "little")]
macro_rules! swap_bytes {
    ($x:expr) => ($x)
}

/// 大端bytes会转为小端bytes 当系统为大端时, 这个宏会生效
#[cfg(target_endian = "big")]
macro_rules! swap_bytes {
    ($x:expr) => ($x.swap_bytes())
}

/// 判断数据类型所需的字节数
macro_rules! type_capacity {
    // u32占4个字节
    (u32) => (4);
    // u64占8个字节
    (u64) => (8)
}

/// vec扩容 计算容量差值 将vec扩容到所需的容量并会更新vec的长度信息
macro_rules! vec_resize {
    ($vec: ident, $len: expr, $offset: expr) => {
        // 偏移量 + 写入的长度 >= vec.len() 需要扩容
        if $offset + $len >= $vec.len() {
            // 扩容操作并不常用, 标记为冷代码
            #[cold]
            {
                // 扩容长度为 偏移量 + 写入长度 与vec.len()的差值
                let add = $offset + $len - $vec.len();
                // 手动扩容 并不一定会扩容, capacity如果剩余容量
                $vec.reserve(add);
                // 需要手动更新容量
                unsafe { $vec.set_len($vec.len() + add); }
            }
        }
    }
}

/// 从MutEncoderData中获取读写指针 如果是MutVector类型,当要写入的长度大于vec容量时会手动扩容
macro_rules! get_mut_ptr {
    // data: 数据容器, len: 要写入的长度(vec时使用), offset: 当前写入的位置(vec时使用)
    ($data: ident, $len: expr, $offset: ident) => {
        match $data {
            MutVector(vec) => {
                let length = $len;
                // vec会进行扩容
                vec_resize!(vec, length, $offset);
                vec.as_mut_ptr()
            }
            MutBuffer(buf) => {
                // buf不进行扩容
                // value的byte数 > buf.len() - offset 时不安全, 内存溢出
                buf.as_mut_ptr()
            }
            MutSlices(slice) => {
                // slice不进行扩容, 直接取切片
                // value的byte数 > slice.size() - offset 时不安全, 内存溢出
                (*slice).as_mut_ptr()
            }
        }
    }
}

/// 从EncoderData中获取只读指针
macro_rules! get_ptr {
    ($data: ident)=>{
        match $data {
            Vector(vec) => {
                vec.as_ptr()
            }
            Buffer(buf) => {
                buf.as_ptr()
            }
            Slices(slice) => {
                (*slice).as_ptr()
            }
        }
    }
}

/// 检查长度 长度不足以写入或者读取时返回错误
macro_rules! check_length {
    ($offset: expr, $write_len: expr, $data_len: expr, write) => {
        // 偏移量 + 写入长度 >= 容器的长度时, 会抛出异常
        if $offset + $write_len > $data_len {
            return Err(LevelError::invalid_argument(
                Slice::from("offset + write_len must < data_len"),
                Slice::from(format!("offset = {}, write_len = {}, data_len = {}", $offset, $write_len, $data_len))));
        }
    };
    ($offset: expr, $limit: expr) => {
        // 偏移量 >= 容器的长度时, 会抛出异常
        if $offset >= $limit {
            return Err(LevelError::invalid_argument(
                Slice::from("offset must < limit"),
                Slice::from(format!("offset = {}, limit = {}", $offset, $limit))
            ));
        }
    };
    ($offset: expr, $read_len: expr, $limit: expr, read) => {
        // 偏移量 + 读取长度 >= 容器的长度时, 会抛出异常
        if $offset + $read_len > $limit {
            return Err(LevelError::invalid_argument(
                Slice::from("offset + read_len must < limit"),
                Slice::from(format!("offset = {}, read_len = {}, limit = {}", $offset, $read_len, $limit))
            ));
        }
    }
}

/// 定长编码
macro_rules! encode_fixed {
    ($name: ident, $type: ty, $capacity: ident) => {
        /// 定长编码 unsafe
        ///
        /// # Safety
        /// * offset < buf/slice的长度, 否则指针越界
        /// * offset + value的字节数 < buf/slice.len(), 否则写数据溢出
        ///
        /// # Arguments
        ///
        /// * `data`: 存放编码数据的容器
        /// * `offset`: 编码的起始偏移量
        /// * `value`: 待编码的数据
        ///
        /// returns: ()
        ///
        /// # Examples
        ///
        /// ```
        ///  let mut vec = vec![];
        ///  // [210, 4, 0, 0]
        ///  unsafe {
        ///     uncheck_encode_fixed32(&mut MutVector(&mut vec), 0, 1234);
        ///  }
        /// ```
        #[inline]
        unsafe fn $name(data: &mut MutEncodeData, offset: usize, value: $type) {
            // 取可变指针
            let mut_ptr = get_mut_ptr!(data, type_capacity!($capacity), offset);
            unsafe {
                // 移动指针
                let end = mut_ptr.add(offset);
                // 写入数据
                ptr::write(end as *mut $type, swap_bytes!(value));
            }
        }
    };
}

encode_fixed!(uncheck_encode_fixed32, u32, u32);
encode_fixed!(uncheck_encode_fixed64, u64, u64);


/// 32位变长编码
///
/// # Safety
/// * offset + 写入字节数 < data.len(), 否则溢出(vec除外)
///
/// # Arguments
///
/// * `data`: 存储编码的数据
/// * `offset`: 编码的偏移量
/// * `value`: 待编码数据
///
/// returns: usize
///
/// # Examples
///
/// ```
///  let mut vec = vec![];
///  let mut offset = 0;
///  // [255, 255, 3]
///  unsafe { offset = uncheck_encode_varint32(&mut MutVector(&mut vec), offset, 65535); }
/// ```
unsafe fn uncheck_encode_varint32(data: &mut MutEncodeData, offset: usize, value: u32) -> usize {
    // 获取varint 需要编码的长度
    let length = varint_length(value as u64);
    // 获取读写指针写入数据
    let mut_ptr = get_mut_ptr!(data, length, offset);

    // 32位字节数较少 直接循环展开
    return if length == 1 {
        ptr::write(mut_ptr.add(offset), value as u8);
        offset + 1
    } else if length == 2 {
        // 直接写入数组 不多次写入一个字节
        // 每次写7个bit不要符号位
        // 最后一位小于128
        ptr::write(mut_ptr.add(offset) as *mut [u8; 2], [
            (value | 128) as u8,
            (value >> 7) as u8,
        ]);
        offset + 2
    } else if length == 3 {
        ptr::write(mut_ptr.add(offset) as *mut [u8; 3], [
            (value | 128) as u8,
            (value >> 7 | 128) as u8,
            (value >> 14) as u8,
        ]);
        offset + 3
    } else if length == 4 {
        ptr::write(mut_ptr.add(offset) as *mut [u8; 4], [
            (value | 128) as u8,
            (value >> 7 | 128) as u8,
            (value >> 14 | 128) as u8,
            (value >> 21) as u8
        ]);
        offset + 4
    } else {
        ptr::write(mut_ptr.add(offset) as *mut [u8; 5], [
            (value | 128) as u8,
            (value >> 7 | 128) as u8,
            (value >> 14 | 128) as u8,
            (value >> 21 | 128) as u8,
            (value >> 28) as u8,
        ]);
        offset + 5
    };
}


/// 64位变长编码
/// # Safety
/// * offset + 写入字节数 < data.len(), 否则溢出(vec除外)
///
/// # Arguments
///
/// * `data`: 存储编码的数据
/// * `offset`: 编码的偏移量
/// * `value`: 待编码数据
///
/// returns: usize
///
/// # Examples
///
/// ```
///  let mut vec = vec![];
///  let mut offset = 0;
///  // offset = 7, vec = [255, 255, 208, 148, 181, 244, 1]
///  unsafe { offset = uncheck_encode_varint64(&mut MutVector(&mut vec), offset, 8_3980_4651_1103) };
/// ```
unsafe fn uncheck_encode_varint64(data: &mut MutEncodeData, mut offset: usize, mut value: u64) -> usize {
    let length = varint_length(value);
    let mut_ptr = get_mut_ptr!(data, length, offset);

    // 每次写7个bit, 如果剩于的值 >= 128, 说明还需要再编码, 最后一位会小于128直接写入即可
    while value >= 128 {
        ptr::write(mut_ptr.add(offset), (value | 128) as u8);
        value >>= 7;
        offset += 1;
    }
    ptr::write(mut_ptr.add(offset), value as u8);
    offset + 1
}

/// 定长解码
macro_rules! decode_fixed {
    {$name: ident, $type: ty} => {
        /// 定长整数解码 不安全
        ///
        /// # Safety
        /// * offset + 读取字节数 < data.len(), 否则溢出
        ///
        /// # Arguments
        ///
        /// * `data`: 待解码数据
        /// * `offset`: 解码位置偏移量
        ///
        /// returns: u32
        ///
        /// # Examples
        ///
        /// ```
        ///  let vec = vec![0, 0, 255, 255];
        ///  // 65535
        ///  let result = unsafe { uncheck_decode_fixed32(&Vector(&vec), 0) };
        /// ```
        #[inline]
        unsafe fn $name(data: &EncodeData, offset: usize) -> $type {
            // offset + 读取字节 >= data.len() 时会溢出
            let ptr = get_ptr!(data);
            swap_bytes!(unsafe {ptr::read(ptr.add(offset) as *mut $type)})
        }
    }
}

decode_fixed!(uncheck_decode_fixed32, u32);
decode_fixed!(uncheck_decode_fixed64, u64);

/// 变长解码
macro_rules! decode_varint {
    ($name: ident, $type: ty, $max_shift: expr) => {
        /// 变长整数解码 不安全
        ///
        /// # Safety
        /// * offset + 读取字节数 < data.len(), 否则溢出
        ///
        /// # Arguments
        ///
        /// * `vec`: 待解码数据
        /// * `offset`: 解码位置偏移量
        ///
        /// returns: u32
        ///
        /// # Examples
        ///
        /// ```
        ///  let vec = vec![255, 255, 3];
        ///  println!("{:?}", vec);
        ///  let mut offset = 0;
        ///  // 65535
        ///  let res = unsafe { uncheck_decode_varint32(&Vector(&vec), offset, vec.len()) };
        /// ```
        unsafe fn $name(data: &EncodeData, mut offset: usize, limit: usize) -> ($type, usize) {
            let ptr = get_ptr!(data);

            // shift的类型是u32, shift为移动的位数, 32位最大28, 64位最大63
            let mut shift = 0 as u32;
            let mut i = offset;
            let mut value = 0 as $type;
            while shift <= $max_shift && i < limit {
                // 解码一个byte
                let byte = unsafe { ptr::read(ptr.add(i)) };
                i += 1;
                // 如果解码的byte > 128, 说明后面还有字节需要继续解码
                if byte & 128 != 0 {
                    value |= (((byte & 127) as $type).overflowing_shl(shift).0) as $type;
                    offset += 1;
                } else {
                    value |= (byte as $type).overflowing_shl(shift).0;
                    offset += 1;
                    return (value, offset);
                }
                shift += 7;
            }
            (value, offset)
        }
    }
}

decode_varint!(uncheck_decode_varint32, u32, 28);
decode_varint!(uncheck_decode_varint64, u64, 63);

/// 写入buf
///
/// # Safety
/// * offset + buf.len() < data.len() , 否则在data不是vec类型的的情况下不会自动扩容, 写入时会溢出
///
/// # Arguments
///
/// * `data`: 存储编码的数据
/// * `offset`: 编码的偏移量
/// * `buf`: 待写入的buf
///
/// returns: ()
///
/// # Examples
///
/// ```
///     let mut vec = vec![];
///
///     let buf = [1, 2, 3, 4, 5];
///     // vec = [1, 2, 3, 4, 5]
///     unsafe { uncheck_write_buf(&mut MutVector(&mut vec), 0, &buf); }
/// ```
unsafe fn uncheck_write_buf(data: &mut MutEncodeData, offset: usize, buf: &[u8]) {
    let mut_ptr = get_mut_ptr!(data, buf.len(), offset).add(offset);
    // 从buf中拷贝数据写入到指针中
    ptr::copy_nonoverlapping(buf.as_ptr(), mut_ptr, buf.len());
}


/// 读取buf 读取时需要知道需要读取的长度
///
/// # Safety
/// * offset + len < data.len() , 否则溢出
///
/// # Arguments
///
/// * `data`: 存储编码的数据
/// * `offset`: 解码的偏移量
///
/// returns: &[u8]
///
/// # Examples
///
/// ```
///     let vec = vec![1, 2, 3, 4, 5, 1, 2, 3, 4];
///     // [1, 2, 3, 4, 5]
///     let buf = unsafe { uncheck_read_buf(&Vector(&vec), 0, 5) };
/// ```
unsafe fn uncheck_read_buf(data: &EncodeData, offset: usize, len: usize) -> Slice {
    let ptr: *const u8 = get_ptr!(data).add(offset);
    // 分配一块内存长度为buf的长度
    let dst: *mut u8 = alloc(Layout::from_size_align_unchecked(len, 4));
    // 将数据拷贝到这块内存上
    intrinsics::copy_nonoverlapping(ptr, dst, len);
    // 使用slice包装内存
    Slice::from_raw_parts(dst, len)
}

/// 编码的数据 只读的
#[derive(Debug)]
enum EncodeData<'a> {
    // vec类型
    Vector(&'a Vec<u8>),
    // buf类型
    Buffer(&'a [u8]),
    // slice类型
    Slices(&'a Slice),
}

/// 编码的数据 可变的
#[derive(Debug)]
enum MutEncodeData<'a> {
    // vec类型, 可以扩容
    MutVector(&'a mut Vec<u8>),
    // buf类型, 不可扩容
    MutBuffer(&'a mut [u8]),
    // slice类型, 不可扩容
    MutSlices(&'a mut Slice),
}

/// 编码器
/// 会维护偏移量, 如果是vec类型会自动扩容
#[derive(Debug)]
pub struct Encoder<'a> {
    // 编码偏移量, 编码时会维护偏移量
    offset: usize,
    // 数据容器
    data: MutEncodeData<'a>,
}

#[derive(Debug)]
pub struct Decoder<'a> {
    // 解码偏移量, 解码时会维护偏移量
    offset: usize,
    // 数据容器
    data: EncodeData<'a>,
    // 最大可解码长度
    limit: usize,
}

/// 实现put_fixed
macro_rules! put_fixed {
    ($name:ident, $var_name:ident, $type:ty, $capacity: ident, uncheck) => {
        /// 编码定长整数 不检查长度
        ///
        /// # Safety
        /// * offset + type_size < data.len() , 否则溢出
        ///
        /// # Arguments
        ///
        /// * `value`: 待编码的数据
        ///
        /// # Examples
        ///
        /// ```
        ///     use level_db_rust::util::coding::Encoder;
        ///     let mut vec = vec![];
        ///     let mut encoder = Encoder::with_vec(&mut vec);
        ///     unsafe {
        ///         // [0, 0, 255, 255]
        ///         encoder.uncheck_put_fixed32(65535);
        ///         // [0, 0, 255, 255, 0, 0, 255, 255]
        ///         encoder.uncheck_put_fixed32(65535);
        ///     }
        /// ```
        pub unsafe fn $name(&mut self, value: $type) {
            // 调用编码方法
            $var_name(&mut self.data, self.offset, value);
            self.offset += type_capacity!($capacity);
        }
    };
    ($name:ident, $var_name:ident, $type:ty, $capacity: ident, check) => {
        /// 编码定长整数 会检查长度
        ///
        /// # Arguments
        ///
        /// * `value`: 待编码的数据
        ///
        /// returns: Result<()>
        ///
        /// # Examples
        ///
        /// ```
        ///     use level_db_rust::util::coding::Encoder;
        ///     let mut vec = vec![];
        ///     let mut encoder = Encoder::with_vec(&mut vec);
        ///     // [0, 0, 255, 255]
        ///     encoder.put_fixed32(65535)?;
        /// ```
        pub fn $name(&mut self, value: $type) -> Result<()> {
            // vec类型自动扩容, buf 和 slice类型检查长度
            if let MutVector(_) = self.data {} else { check_length!(self.offset, type_capacity!($capacity), self.len(), write) };
            // 调用编码方法
            unsafe {$var_name(&mut self.data, self.offset, value);}
            self.offset += type_capacity!($capacity);
            Ok(())
        }
    };
}

/// 实现put_varint
macro_rules! put_varint {
    ($name:ident, $var_name:ident, $type:ty, uncheck) => {
        /// 编码变长整数 不检查长度
        ///
        /// # Safety
        /// * offset + varint_length < data.len() , 否则溢出
        ///
        /// # Arguments
        ///
        /// * `value`: 待编码的数据
        ///
        /// # Examples
        ///
        /// ```
        ///     use level_db_rust::util::coding::Encoder;
        ///     let mut vec = vec![];
        ///     let mut encoder = Encoder::with_vec(&mut vec);
        ///     unsafe {
        ///         // [255, 255, 3]
        ///         encoder.uncheck_put_varint32(65535);
        ///         // [255, 255, 3, 255, 255, 3]
        ///         encoder.uncheck_put_varint64(65535);
        ///     }
        /// ```
        pub unsafe fn $name(&mut self, value: $type) {
            // 调用编码方法
            self.offset = $var_name(&mut self.data, self.offset, value);
        }
    };
    ($name:ident, $var_name:ident, $type:ty, check) => {
        /// 编码变长整数 会检查长度
        ///
        /// # Arguments
        ///
        /// * `value`: 待编码的数据
        ///
        /// # Examples
        ///
        /// ```
        ///     use level_db_rust::util::coding::Encoder;
        ///     let mut vec = vec![];
        ///     let mut encoder = Encoder::with_vec(&mut vec);
        ///     // [255, 255, 3]
        ///     encoder.put_varint32(65535)?;
        ///     // [255, 255, 3, 255, 255, 3]
        ///     encoder.put_varint64(65535)?;
        /// ```
        pub fn $name(&mut self, value: $type) -> Result<()> {
            // vec类型自动扩容, buf 和 slice类型检查长度
            if let MutVector(_) = self.data {} else { check_length!(self.offset, varint_length(value as u64), self.len(), write) };
            // 调用编码方法
            unsafe { self.offset = $var_name(&mut self.data, self.offset, value) }
            Ok(())
        }
    }
}

impl<'a> Encoder<'a> {
    /// 以vec做为容器生成encoder
    /// 编码时当容量不足时会扩容
    /// 如果以追加的方式进行编码推荐使用vec做为容器
    /// 使用vec容器时, 推荐使用uncheck的方法
    ///
    /// # Arguments
    ///
    /// * `vec`: vec
    ///
    /// returns: Encoder
    ///
    /// # Examples
    ///
    /// ```
    ///  use level_db_rust::util::coding::Encoder;
    ///  let mut vec = vec![];
    ///  let mut encoder = Encoder::with_vec(&mut vec);
    /// ```
    pub fn with_vec(vec: &'a mut Vec<u8>) -> Self {
        Self {
            offset: 0,
            data: MutVector(vec),
        }
    }
    /// 以切片做为容器生成encoder
    /// 编码时当容量不足时可能会造成内存溢出
    /// 需要提前规划好需要使用的容量, 并保证调用编码方式时不会溢出
    ///
    /// # Arguments
    ///
    /// * `buf`: buf
    ///
    /// returns: Encoder
    ///
    /// # Examples
    ///
    /// ```
    ///  use level_db_rust::util::coding::Encoder;
    ///  let mut buf = [0; 20];
    ///  unsafe {
    ///     let mut encoder = Encoder::with_buf(&mut buf);
    ///  }
    /// ```
    pub fn with_buf(buf: &'a mut [u8]) -> Self {
        Self {
            offset: 0,
            data: MutBuffer(buf),
        }
    }

    /// 以slice做为容器生成encoder
    /// 编码时当容量不足时会溢出
    ///
    /// # Arguments
    ///
    /// * `slice`: slice
    ///
    /// returns: Encoder
    ///
    /// # Examples
    ///
    /// ```
    ///  use level_db_rust::util::coding::Encoder;
    ///  use level_db_rust::util::slice::Slice;
    ///  let mut slice = Slice::from_vec(vec![0; 20]);
    ///  unsafe {
    ///     let mut encoder = Encoder::with_slice(&mut slice);
    ///  }
    /// ```
    pub fn with_slice(slice: &'a mut Slice) -> Self {
        Self {
            offset: 0,
            data: MutSlices(slice),
        }
    }

    /// 从encoder的数据中生成decoder
    ///
    /// returns: Decoder
    ///
    /// # Examples
    ///
    /// ```
    ///     use level_db_rust::util::coding::Encoder;
    ///     let mut data = vec![1, 2, 3];
    ///     let encoder = Encoder::with_vec(&mut data);
    ///     let decoder = encoder.create_decoder();
    /// ```
    pub fn create_decoder(&'a self) -> Decoder<'a> {
        Decoder::from_encoder(self)
    }

    put_fixed!(uncheck_put_fixed32, uncheck_encode_fixed32, u32, u32, uncheck);
    put_fixed!(uncheck_put_fixed64, uncheck_encode_fixed64, u64, u64, uncheck);
    put_fixed!(put_fixed32, uncheck_encode_fixed32, u32, u32, check);
    put_fixed!(put_fixed64, uncheck_encode_fixed64, u64, u64, check);

    put_varint!(uncheck_put_varint32, uncheck_encode_varint32, u32, uncheck);
    put_varint!(uncheck_put_varint64, uncheck_encode_varint64, u64, uncheck);
    put_varint!(put_varint32, uncheck_encode_varint32, u32, check);
    put_varint!(put_varint64, uncheck_encode_varint64, u64, check);

    /// 向encoder中直接写入数据不用进行编码
    /// 向vec中写入时会自动扩容
    /// # Safety
    /// * self.offset + buf.len() > self.data , 如果data不是vec的话不会自动扩容, 会溢出
    ///
    /// # Arguments
    ///
    /// * `buf`: 待写入的数据
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///     use level_db_rust::util::coding::Encoder;
    ///     let mut vec = vec![];
    ///     let mut encoder = Encoder::with_vec(&mut vec);
    ///     let buf = [1, 2, 3];
    ///     // vec: [1, 2, 3]
    ///     unsafe { encoder.uncheck_put_buf(&buf) }
    /// ```
    pub unsafe fn uncheck_put_buf(&mut self, buf: &[u8]) {
        uncheck_write_buf(&mut self.data, self.offset, buf);
        self.offset += buf.len();
    }

    /// 向encoder中直接写入数据不用进行编码
    /// 向vec中写入时会自动扩容
    /// 会检查是否能够写入
    ///
    /// # Arguments
    ///
    /// * `buf`: 待写入的数据
    ///
    /// returns: Result<(), Status>
    ///
    /// # Examples
    ///
    /// ```
    ///     use level_db_rust::util::coding::Encoder;
    ///     let mut vec = vec![];
    ///     let mut encoder = Encoder::with_vec(&mut vec);
    ///     let buf = [1, 2, 3];
    ///     // vec: [1, 2, 3]
    ///     encoder.put_buf(&buf)?
    /// ```
    pub fn put_buf(&mut self, buf: &[u8]) -> Result<()> {
        // vec类型自动扩容 buf 和 slice类型检查长度
        if let MutVector(_) = self.data {} else { check_length!(self.offset, buf.len(), self.len(), write) };
        unsafe { uncheck_write_buf(&mut self.data, self.offset, buf); }
        self.offset += buf.len();
        Ok(())
    }

    /// 写入slice时先写入slice的长度做为前缀
    /// slice(data:[1,2,3],size:3), 写入后[3,1,2,3]
    ///
    /// # Safety
    /// * u32的字节数(4) + slice的字节数(slice.size()) < self.data.len(), 否则溢出(vec除外)
    ///
    /// # Arguments
    ///
    /// * `slice`: 待写入的slice
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///  use level_db_rust::util::coding::Encoder;
    ///  use level_db_rust::util::slice::Slice;
    ///  let mut vec = vec![];
    ///  let mut encoder = Encoder::with_vec(&mut vec);
    ///  let slice = Slice::from_vec(vec![1, 2, 3]);
    ///  // vec: [3, 1, 2, 3]
    ///  // The first '3' of the vec is the length of the slice,
    ///  // and the following '1,2,3' is the data of the slice
    ///  unsafe { encoder.uncheck_put_length_prefixed_slice(&slice); }
    /// ```
    pub unsafe fn uncheck_put_length_prefixed_slice(&mut self, slice: &Slice) {
        self.uncheck_put_varint32(slice.size() as u32);
        self.uncheck_put_buf(slice);
    }

    /// 写入slice时先写入slice的长度做为前缀
    ///
    /// # Arguments
    ///
    /// * `slice`: 待写入的slice
    ///
    /// returns: Result<(), Status>
    ///
    /// # Examples
    ///
    /// ```
    ///  use level_db_rust::util::coding::Encoder;
    ///  use level_db_rust::util::slice::Slice;
    ///  let mut vec = vec![];
    ///  let mut encoder = Encoder::with_vec(&mut vec);
    ///  let slice = Slice::from_vec(vec![1, 2, 3]);
    ///  // vec: [3, 1, 2, 3]
    ///  // The first '3' of the vec is the length of the slice,
    ///  // and the following '1,2,3' is the data of the slice
    ///  encoder.put_length_prefixed_slice(&slice)?;
    /// ```
    pub fn put_length_prefixed_slice(&mut self, slice: &Slice) -> Result<()> {
        self.put_varint32(slice.size() as u32)?;
        self.put_buf(slice)?;
        Ok(())
    }

    /// 获取当前编码到的位置
    ///
    /// returns: usize
    ///
    /// # Examples
    ///
    /// ```
    /// use level_db_rust::util::coding::Encoder;
    /// let mut  vec = vec![];
    /// let mut encoder = Encoder::with_vec(&mut vec);
    /// // offset: 0
    /// let offset = encoder.offset();
    /// encoder.put_varint32(65535)?;
    /// // offset: 3
    /// let offset = encoder.offset();
    /// ```
    #[inline]
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// 获取容器的长度
    ///
    /// returns: usize
    ///
    /// # Examples
    ///
    /// ```
    /// use level_db_rust::util::coding::Encoder;
    /// let mut  vec = vec![];
    /// let mut encoder = Encoder::with_vec(&mut vec);
    /// // len: 0
    /// let len = encoder.len();
    /// encoder.put_varint32(65535)?;
    /// // len: 3
    /// let len = encoder.len();
    /// ```
    pub fn len(&self) -> usize {
        match self.data {
            MutVector(ref vec) => {
                vec.len()
            }
            MutBuffer(ref buf) => {
                buf.len()
            }
            MutSlices(ref slice) => {
                slice.size()
            }
        }
    }
}

macro_rules! get_fixed {
    ($name:ident, $var_name:ident, $type:ty, $capacity: ident, uncheck) => {
        /// 定长解码
        /// # Safety
        /// * self.offset < self.limit 先调用 encoder.can_get() 确定可以解码再调用, 否则溢出
        ///
        /// returns: u32/u64
        ///
        /// # Examples
        ///
        /// ```
        /// use level_db_rust::util::coding::Decoder;
        /// let mut vec = vec![0, 0, 255, 255];
        /// let mut decoder = Decoder::with_vec(&mut vec);
        /// // 65535
        /// let value = unsafe { decoder.uncheck_get_fixed32() };
        /// ```
        #[inline]
        pub unsafe fn $name(&mut self) -> $type {
            // 调用解码方法
            let value = $var_name(&self.data, self.offset);
            self.offset += type_capacity!($capacity);
            value
        }
    };
    ($name:ident, $var_name:ident, $type:ty, $capacity: ident, check) => {
        /// 定长解码
        /// # Safety
        /// * self.offset < self.limit 先调用 encoder.can_get() 确定可以解码再调用 否则溢出
        ///
        /// returns: u32/u64
        ///
        /// # Examples
        ///
        /// ```
        /// use level_db_rust::util::coding::Decoder;
        /// let mut vec = vec![0, 0, 255, 255];
        /// let mut decoder = Decoder::with_vec(&mut vec);
        /// // 65535
        /// let value = decoder.get_fixed32()?;
        /// ```
        #[inline]
        pub fn $name(&mut self) -> Result<$type> {
            check_length!(self.offset, type_capacity!($capacity), self.limit, read);
            // 调用解码方法
            let value = unsafe { $var_name(&self.data, self.offset) };
            self.offset += type_capacity!($capacity);
            Ok(value)
        }
    }
}

macro_rules! get_varint {
    ($name:ident, $var_name:ident, $type:ty, uncheck) => {
        /// 变长解码
        ///
        /// returns: u32/u64
        ///
        /// # Examples
        ///
        /// ```
        /// use level_db_rust::util::coding::Decoder;
        /// let mut vec = vec![255, 255, 3];
        /// let mut decoder = Decoder::with_vec(&mut vec);
        /// // 65535
        /// let value = unsafe { decoder.uncheck_get_varint32() };
        /// ```
        #[inline]
        pub unsafe fn $name(&mut self) -> $type {
            // 调用解码方法
            let res = $var_name(&self.data, self.offset, self.limit);
            self.offset = res.1;
            res.0
        }
    };
    ($name:ident, $var_name:ident, $type:ty, check) => {
        /// 变长解码
        ///
        /// returns: u32/u64
        ///
        /// # Examples
        ///
        /// ```
        /// use level_db_rust::util::coding::Decoder;
        /// let mut vec = vec![255, 255, 3];
        /// let mut decoder = Decoder::with_vec(&mut vec);
        /// // 65535
        /// let value = decoder.get_varint32()?;
        /// ```
        #[inline]
        pub fn $name(&mut self) -> Result<$type> {
            check_length!(self.offset, self.limit);
            // 调用解码方法
            let res = unsafe { $var_name(&self.data, self.offset, self.limit) };
            self.offset = res.1;
            Ok(res.0)
        }
    }
}

impl<'a> Decoder<'a> {
    pub fn with_slice(slice: &'a Slice) -> Self {
        Self {
            offset: 0,
            limit: slice.size(),
            data: Slices(slice),
        }
    }
    pub fn with_buf(buf: &'a [u8]) -> Self {
        Self {
            offset: 0,
            limit: buf.len(),
            data: Buffer(buf),
        }
    }
    pub fn with_vec(vec: &'a Vec<u8>) -> Self {
        Self {
            offset: 0,
            data: Vector(vec),
            limit: vec.len(),
        }
    }

    pub fn from_encoder(encoder: &'a Encoder) -> Self {
        Self {
            offset: 0,
            limit: encoder.len(),
            data: match encoder.data {
                MutVector(ref vec) => {
                    Vector(vec)
                }
                MutBuffer(ref buf) => {
                    Buffer(buf)
                }
                MutSlices(ref slice) => {
                    Slices(slice)
                }
            },
        }
    }

    /// 判断是否有数据可以读取
    /// 数据读取到末尾 不满足 offset < limit 时为false
    /// 如果使用了uncheck的方法 需要调用这个方法判断是否可以读取 否则可能会溢出
    ///
    /// returns: bool
    ///
    /// # Examples
    ///
    /// ```
    /// use level_db_rust::util::coding::Decoder;
    /// let mut vec = vec![255, 255, 3];
    /// let mut decoder = Decoder::with_vec(&mut vec);
    /// // true
    /// let can_get = decoder.can_get();
    /// decoder.get_varint32()?;
    /// // false
    /// let can_get = decoder.can_get();
    /// ```
    #[inline]
    pub fn can_get(&self) -> bool {
        self.offset < self.limit
    }

    get_fixed!(uncheck_get_fixed32, uncheck_decode_fixed32, u32, u32, uncheck);
    get_fixed!(uncheck_get_fixed64, uncheck_decode_fixed64, u64, u64, uncheck);
    get_fixed!(get_fixed32, uncheck_decode_fixed32, u32, u32, check);
    get_fixed!(get_fixed64, uncheck_decode_fixed64, u64, u64, check);

    get_varint!(uncheck_get_varint32, uncheck_decode_varint32, u32, uncheck);
    get_varint!(uncheck_get_varint64, uncheck_decode_varint64, u64, uncheck);
    get_varint!(get_varint32, uncheck_decode_varint32, u32, check);
    get_varint!(get_varint64, uncheck_decode_varint64, u64, check);

    /// 解码出slice 不检查长度
    ///
    /// # Safety
    /// * self.offset < self.len() , 否则溢出
    ///
    /// returns: Slice
    ///
    /// # Examples
    ///
    /// ```
    ///  use level_db_rust::util::coding::Decoder;
    ///  let vec = vec![3, 1, 2, 3];
    ///  let mut decoder = Decoder::with_vec(&vec);
    ///  // [1, 2, 3]
    ///  let slice = unsafe { decoder.uncheck_get_length_prefixed_slice() };
    /// ```
    pub unsafe fn uncheck_get_length_prefixed_slice(&mut self) -> Slice {
        let size = self.uncheck_get_varint32() as usize;
        self.uncheck_get_buf(size)
    }

    /// 解码出slice
    ///
    /// returns: Result<Slice>
    ///
    /// # Examples
    ///
    /// ```
    ///  use level_db_rust::util::coding::Decoder;
    ///  let vec = vec![3, 1, 2, 3];
    ///  let mut decoder = Decoder::with_vec(&vec);
    ///  // [1, 2, 3]
    ///  let slice = decoder.get_length_prefixed_slice()?;
    /// ```
    pub fn get_length_prefixed_slice(&mut self) -> Result<Slice> {
        check_length!(self.offset, self.limit);
        let size = unsafe { self.uncheck_get_varint32() } as usize;
        check_length!(self.offset, size, self.limit, read);
        unsafe { Ok(self.uncheck_get_buf(size)) }
    }

    /// 获取buf 不检查长度
    ///
    /// # Safety
    /// * self.offset + len < self.limit, 否则溢出
    ///
    /// # Arguments
    ///
    /// * `data`: 待解码数据
    /// * `len`: 解码buf的长度, 必须要指定的, 否则无法正确读取
    ///
    /// returns: Slice
    ///
    /// # Examples
    ///
    /// ```
    ///  use level_db_rust::util::coding::Decoder;
    ///  let vec = vec![1, 2, 3];
    ///  let mut decoder = Decoder::with_vec(&vec);
    ///  // [1, 2, 3]
    ///  let buf = unsafe { decoder.uncheck_get_buf(3) };
    /// ```
    pub unsafe fn uncheck_get_buf(&mut self, len: usize) -> Slice {
        let slice = uncheck_read_buf(&self.data, self.offset, len);
        self.offset += len;
        slice
    }

    /// 读取buf
    ///
    /// # Arguments
    ///
    /// * `data`: 待解码数据
    /// * `len`: 读取buf的长度, 必须要指定的, 否则无法正确读取
    ///
    /// returns: Result<Slice>, Status>
    ///
    /// # Examples
    ///
    /// ```
    ///  use level_db_rust::util::coding::Decoder;
    ///  let vec = vec![1, 2, 3];
    ///  let mut decoder = Decoder::with_vec(&vec);
    ///  // [1, 2, 3]
    ///  let buf = decoder.get_buf(3)?;
    /// ```
    pub fn get_buf(&self, len: usize) -> Result<Slice> {
        check_length!(self.offset, len, self.limit, read);
        unsafe {
            Ok(uncheck_read_buf(&self.data, self.offset, len))
        }
    }

    /// 跳过一段长度 偏移量会移动到跳过后的位置继续读取 未检查偏移量
    ///
    /// # Safety
    /// * offset + skip < self.limit, 否则会出现未定义行为, 读取将溢出
    ///
    /// # Arguments
    ///
    /// * `skip`: 需要跳过的长度
    ///
    /// returns: usize
    ///
    /// # Examples
    ///
    /// ```
    /// use level_db_rust::util::coding::Decoder;
    /// let vec = vec![255, 1, 255, 255, 3];
    /// // offset: 0
    /// let mut decoder = Decoder::with_vec(&vec);
    /// // offset: 2
    /// unsafe { decoder.uncheck_skip(2) };
    /// // value: 65535
    /// let value = decoder.get_varint32()?;
    /// ```
    pub unsafe fn uncheck_skip(&mut self, skip: usize) -> usize {
        self.offset += skip;
        self.offset
    }

    /// 跳过一段长度 偏移量会移动到跳过后的位置继续读取
    ///
    /// # Arguments
    ///
    /// * `skip`: 需要跳过的长度
    ///
    /// returns: Result<usize, Status>
    ///
    /// # Examples
    ///
    /// ```
    /// use level_db_rust::util::coding::Decoder;
    /// let vec = vec![255, 1, 255, 255, 3];
    /// // offset: 0
    /// let mut decoder = Decoder::with_vec(&vec);
    /// // offset: 2
    /// decoder.skip(2)?;
    /// // value: 65535
    /// let value = decoder.get_varint32()?;
    /// ```
    pub fn skip(&mut self, skip: usize) -> Result<usize> {
        check_length!(self.offset, skip, self.limit, read);
        self.offset += skip;
        Ok(self.offset)
    }

    /// 获取当前编码到的位置
    ///
    /// returns: usize
    ///
    /// # Examples
    ///
    /// ```
    /// use level_db_rust::util::coding::Decoder;
    /// let vec = vec![255, 1, 255, 255, 3];
    /// let mut decoder = Decoder::with_vec(&vec);
    /// // offset: 0
    /// let value = decoder.get_varint32()?;
    /// // offset: 2
    /// let offset = decoder.offset();
    /// ```
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// 获取编码数据的可解码限制
    /// offset < limit
    ///
    /// returns: usize
    ///
    /// # Examples
    ///
    /// ```
    /// use level_db_rust::util::coding::Decoder;
    /// let vec = vec![255, 1, 255, 255, 3];
    /// let mut decoder = Decoder::with_vec(&vec);
    /// // limit: 5
    /// let limit = decoder.limit();
    /// ```
    pub fn limit(&self) -> usize {
        self.limit
    }
}

#[test]
fn test_varint_length() {
    let length = varint_length(1);
    assert_eq!(1, length);
    let length = varint_length(127);
    assert_eq!(1, length);
    let length = varint_length(128);
    assert_eq!(2, length);
    let length = varint_length(255);
    assert_eq!(2, length);
    let length = varint_length(16383);
    assert_eq!(2, length);
    let length = varint_length(16384);
    assert_eq!(3, length);
    let length = varint_length(65535);
    assert_eq!(3, length);
    let length = varint_length(209_7151);
    assert_eq!(3, length);
    let length = varint_length(209_7152);
    assert_eq!(4, length);
    let length = varint_length(2_6843_5455);
    assert_eq!(4, length);
    let length = varint_length(2_6843_5456);
    assert_eq!(5, length);
    // 1 << 35
    let length = varint_length(343_5973_8367);
    assert_eq!(5, length);
    let length = varint_length(343_5973_8368);
    assert_eq!(6, length);
    let length = varint_length(4_3980_4651_1103);
    assert_eq!(6, length);
    let length = varint_length(4_3980_4651_1104);
    assert_eq!(7, length);
    let length = varint_length(562_9499_5342_1311);
    assert_eq!(7, length);
    let length = varint_length(562_9499_5342_1312);
    assert_eq!(8, length);
    let length = varint_length(7_2057_5940_3792_7935);
    assert_eq!(8, length);
    let length = varint_length(7_2057_5940_3792_7936);
    assert_eq!(9, length);
    let length = varint_length(922_3372_0368_5477_5807);
    assert_eq!(9, length);
    let length = varint_length(922_3372_0368_5477_5808);
    assert_eq!(10, length);
}

#[test]
fn test_encode_fixed() {
    let mut vec = vec![];
    unsafe { uncheck_encode_fixed32(&mut MutVector(&mut vec), 0, 1234); }
    println!("{:?}", vec);
    assert_eq!(vec![210, 4, 0, 0], vec);
    assert_eq!(4, vec.len());

    unsafe { uncheck_encode_fixed32(&mut MutVector(&mut vec), 4, 3_0000_0000); }
    println!("{:?}", vec);
    assert_eq!(8, vec.len());
    assert_eq!(vec![210, 4, 0, 0, 0, 163, 225, 17], vec);

    let mut vec = vec![];
    unsafe { uncheck_encode_fixed64(&mut MutVector(&mut vec), 0, 8_3980_4651_1103); }
    println!("{:?}", vec);
    assert_eq!(8, vec.len());

    unsafe { uncheck_encode_fixed64(&mut MutVector(&mut vec), 8, 900_3372_0368_5477_5808); }
    println!("{:?}", vec);
    assert_eq!(16, vec.len());
    assert_eq!(vec![255, 63, 148, 82, 163, 7, 0, 0, 0, 0, 106, 101, 42, 103, 242, 124], vec);
}

#[test]
fn test_decode_fixed() {
    let mut vec = vec![];
    unsafe {
        uncheck_encode_fixed32(&mut MutVector(&mut vec), 0, 1234);
        uncheck_encode_fixed32(&mut MutVector(&mut vec), 4, 128);
        uncheck_encode_fixed32(&mut MutVector(&mut vec), 8, 255);
        uncheck_encode_fixed32(&mut MutVector(&mut vec), 12, 65535);
        uncheck_encode_fixed32(&mut MutVector(&mut vec), 16, 10000000);
    }
    println!("{:?}", vec);
    assert_eq!(vec![210, 4, 0, 0, 128, 0, 0, 0, 255, 0, 0, 0, 255, 255, 0, 0, 128, 150, 152, 0], vec);

    let result = unsafe { uncheck_decode_fixed32(&Vector(&vec), 0) };
    println!("{}", result);
    assert_eq!(1234, result);

    let result = unsafe { uncheck_decode_fixed32(&Vector(&vec), 4) };
    println!("{}", result);
    assert_eq!(128, result);

    let result = unsafe { uncheck_decode_fixed32(&Vector(&vec), 8) };
    println!("{}", result);
    assert_eq!(255, result);

    let result = unsafe { uncheck_decode_fixed32(&Vector(&vec), 12) };
    println!("{}", result);
    assert_eq!(65535, result);

    let result = unsafe { uncheck_decode_fixed32(&Vector(&vec), 16) };
    println!("{}", result);
    assert_eq!(10000000, result);

    let mut vec = vec![];
    unsafe {
        uncheck_encode_fixed64(&mut MutVector(&mut vec), 0, 8_3980_4651_1103);
        uncheck_encode_fixed64(&mut MutVector(&mut vec), 8, 900_3372_0368_5477_5808);
    }
    println!("{:?}", vec);
    assert_eq!(vec![255, 63, 148, 82, 163, 7, 0, 0, 0, 0, 106, 101, 42, 103, 242, 124], vec);

    let result = unsafe { uncheck_decode_fixed64(&Vector(&vec), 0) };
    println!("{}", result);
    assert_eq!(8_3980_4651_1103, result);

    let result = unsafe { uncheck_decode_fixed64(&Vector(&vec), 8) };
    println!("{}", result);
    assert_eq!(900_3372_0368_5477_5808, result);
}

#[test]
fn test_encode_varint() {
    let mut vec = vec![];
    let mut offset = 0;
    unsafe { offset = uncheck_encode_varint32(&mut MutVector(&mut vec), offset, 2); }
    println!("{:?}", vec);
    println!("offset: {}", offset);

    unsafe { offset = uncheck_encode_varint32(&mut MutVector(&mut vec), offset, 128); }
    println!("{:?}", vec);
    println!("offset: {}", offset);

    unsafe { offset = uncheck_encode_varint32(&mut MutVector(&mut vec), offset, 255); }
    println!("{:?}", vec);
    println!("offset: {}", offset);

    unsafe { offset = uncheck_encode_varint32(&mut MutVector(&mut vec), offset, 65535); }
    println!("{:?}", vec);
    println!("offset: {}", offset);

    unsafe { offset = uncheck_encode_varint32(&mut MutVector(&mut vec), offset, 10000000); }
    println!("{:?}", vec);
    println!("offset: {}", offset);

    unsafe { offset = uncheck_encode_varint32(&mut MutVector(&mut vec), offset, 209_7152); }
    println!("{:?}", vec);
    println!("offset: {}", offset);

    unsafe { offset = uncheck_encode_varint32(&mut MutVector(&mut vec), offset, 2_6843_5456); }
    println!("{:?}", vec);
    println!("offset: {}", offset);

    assert_eq!(21, offset);
    assert_eq!(vec![2, 128, 1, 255, 1, 255, 255, 3, 128, 173, 226, 4, 128, 128, 128, 1, 128, 128, 128, 128, 1], vec);

    let mut vec = vec![];
    let mut offset = 0;

    unsafe { offset = uncheck_encode_varint64(&mut MutVector(&mut vec), offset, 65535) };
    println!("{:?}", vec);
    println!("offset: {}", offset);

    unsafe { offset = uncheck_encode_varint64(&mut MutVector(&mut vec), offset, 8_3980_4651_1103) };
    println!("{:?}", vec);
    println!("offset: {}", offset);

    unsafe { offset = uncheck_encode_varint64(&mut MutVector(&mut vec), offset, 900_3372_0368_5477_5808) };
    println!("{:?}", vec);
    println!("offset: {}", offset);

    assert_eq!(19, offset);
    assert_eq!(vec![255, 255, 3, 255, 255, 208, 148, 181, 244, 1, 128, 128, 168, 171, 166, 229, 153, 249, 124], vec);
}

#[test]
fn test_decode_varint() {
    let vec = vec![2, 128, 1, 255, 1, 255, 255, 3, 128, 173, 226, 4, 128, 128, 128, 1, 128, 128, 128, 128, 1];
    println!("{:?}", vec);
    let mut offset = 0;
    let res = unsafe { uncheck_decode_varint32(&Vector(&vec), offset, vec.len()) };
    offset = res.1;
    println!("value: {}", res.0);
    println!("offset: {}", offset);
    assert_eq!(res.0, 2);

    let res = unsafe { uncheck_decode_varint32(&Vector(&vec), offset, vec.len()) };
    offset = res.1;
    println!("value: {}", res.0);
    println!("offset: {}", offset);
    assert_eq!(res.0, 128);

    let res = unsafe { uncheck_decode_varint32(&Vector(&vec), offset, vec.len()) };
    offset = res.1;
    println!("value: {}", res.0);
    println!("offset: {}", offset);
    assert_eq!(res.0, 255);

    let res = unsafe { uncheck_decode_varint32(&Vector(&vec), offset, vec.len()) };
    offset = res.1;
    println!("value: {}", res.0);
    println!("offset: {}", offset);
    assert_eq!(res.0, 65535);

    let res = unsafe { uncheck_decode_varint32(&Vector(&vec), offset, vec.len()) };
    offset = res.1;
    println!("value: {}", res.0);
    println!("offset: {}", offset);
    assert_eq!(res.0, 10000000);

    let res = unsafe { uncheck_decode_varint32(&Vector(&vec), offset, vec.len()) };
    offset = res.1;
    println!("value: {}", res.0);
    println!("offset: {}", offset);
    assert_eq!(res.0, 209_7152);

    let res = unsafe { uncheck_decode_varint32(&Vector(&vec), offset, vec.len()) };
    offset = res.1;
    println!("value: {}", res.0);
    println!("offset: {}", offset);
    assert_eq!(res.0, 2_6843_5456);

    println!("decode varint64: ");
    let vec = vec![255, 255, 3, 255, 255, 208, 148, 181, 244, 1, 128, 128, 168, 171, 166, 229, 153, 249, 124];
    println!("{:?}", vec);
    let mut offset = 0;
    let res = unsafe { uncheck_decode_varint64(&Vector(&vec), offset, vec.len()) };
    offset = res.1;
    println!("value: {}", res.0);
    println!("offset: {}", offset);
    assert_eq!(65535, res.0);

    let res = unsafe { uncheck_decode_varint64(&Vector(&vec), offset, vec.len()) };
    offset = res.1;
    println!("value: {}", res.0);
    println!("offset: {}", offset);
    assert_eq!(8_3980_4651_1103, res.0);

    let res = unsafe { uncheck_decode_varint64(&Vector(&vec), offset, vec.len()) };
    offset = res.1;
    println!("value: {}", res.0);
    println!("offset: {}", offset);
    assert_eq!(900_3372_0368_5477_5808, res.0);
}


#[test]
fn test_write_buf() {
    let mut vec = vec![];

    let buf = [1, 2, 3, 4, 5];
    unsafe { uncheck_write_buf(&mut MutVector(&mut vec), 0, &buf); }

    println!("{:?}", vec);
    assert_eq!(vec![1, 2, 3, 4, 5], vec);

    let buf = [1, 2, 3, 4];
    unsafe { uncheck_write_buf(&mut MutVector(&mut vec), 5, &buf); }

    println!("{:?}", vec);
    assert_eq!(vec![1, 2, 3, 4, 5, 1, 2, 3, 4], vec);
}

#[test]
fn test_read_buf() {
    let vec = vec![1, 2, 3, 4, 5, 1, 2, 3, 4];
    let buf = unsafe { uncheck_read_buf(&Vector(&vec), 0, 5) };
    println!("{:?}", buf);
    assert_eq!(&[1_u8, 2, 3, 4, 5] as &[u8; 5], buf.deref());
    let buf = unsafe { uncheck_read_buf(&Vector(&vec), 5, 4) };
    println!("{:?}", buf);
    assert_eq!(&[1_u8, 2, 3, 4] as &[u8; 4], buf.deref());
}

#[test]
fn test_mixed_encode_decode() {
    // 混合类型编码 解码  varint32 varint64 fixed32 fixed64 write_buf read_buf
    let mut vec = vec![];
    let mut offset = 0;
    unsafe { uncheck_encode_fixed32(&mut MutVector(&mut vec), offset, 3) };
    offset += 4;
    offset = unsafe { uncheck_encode_varint32(&mut MutVector(&mut vec), offset, 655535) };
    unsafe { uncheck_encode_fixed64(&mut MutVector(&mut vec), offset, 7) };
    offset += 8;
    offset = unsafe { uncheck_encode_varint64(&mut MutVector(&mut vec), offset, 8_3980_4651_1103) };
    let buf = [1, 2, 3, 4];
    unsafe { uncheck_write_buf(&mut MutVector(&mut vec), offset, &buf) };
    offset += buf.len();
    println!("{:?}", vec);
    println!("offset: {}", offset);

    offset = 0;
    let value = unsafe { uncheck_decode_fixed32(&Vector(&vec), offset) };
    println!("{}", value);
    assert_eq!(3, value);
    offset += 4;
    let res = unsafe { uncheck_decode_varint32(&Vector(&vec), offset, (&vec).len()) };
    println!("{}", res.0);
    assert_eq!(655535, res.0);
    offset = res.1;
    let value = unsafe { uncheck_decode_fixed64(&Vector(&vec), offset) };
    println!("{}", value);
    assert_eq!(7, value);
    offset += 8;
    let res = unsafe { uncheck_decode_varint64(&Vector(&vec), offset, (&vec).len()) };
    println!("{}", res.0);
    assert_eq!(8_3980_4651_1103, res.0);
    offset = res.1;

    let buf = unsafe { uncheck_read_buf(&Vector(&vec), offset, 4) };
    println!("{:?}", buf);
    assert_eq!(&[1_u8, 2, 3, 4] as &[u8; 4], buf.deref());

    println!("offset: {}", offset);
    assert_eq!(22, offset);
}

#[test]
fn test_put_fixed() -> Result<()> {
    let mut vec = vec![];

    unsafe {
        let mut encoder = Encoder::with_vec(&mut vec);
        println!("{:?}", encoder);
        encoder.uncheck_put_fixed32(2);
        encoder.uncheck_put_fixed32(128);
        encoder.uncheck_put_fixed32(255);
        encoder.uncheck_put_fixed32(65535);
        encoder.uncheck_put_fixed32(10000000);
        encoder.uncheck_put_fixed64(655535);
        encoder.uncheck_put_fixed64(8_3980_4651_1103);
        encoder.uncheck_put_fixed64(900_3372_0368_5477_5808);
        println!("{:?}", &encoder);
        if let MutVector(data) = encoder.data {
            assert_eq!(&mut vec![
                2, 0, 0, 0, 128, 0, 0, 0, 255, 0, 0, 0, 255, 255, 0, 0, 128, 150, 152, 0, 175, 0,
                10, 0, 0, 0, 0, 0, 255, 63, 148, 82, 163, 7, 0, 0, 0, 0, 106, 101, 42, 103, 242, 124
            ],
                       data);
        }
    }

    let mut encoder = Encoder::with_vec(&mut vec);
    println!("{:?}", encoder);
    encoder.put_fixed32(2)?;
    encoder.put_fixed32(128)?;
    encoder.put_fixed32(255)?;
    encoder.put_fixed32(65535)?;
    encoder.put_fixed32(10000000)?;
    encoder.put_fixed64(655535)?;
    encoder.put_fixed64(8_3980_4651_1103)?;
    encoder.put_fixed64(900_3372_0368_5477_5808)?;
    println!("{:?}", &encoder);
    if let MutVector(data) = encoder.data {
        assert_eq!(&mut vec![
            2, 0, 0, 0, 128, 0, 0, 0, 255, 0, 0, 0, 255, 255, 0, 0, 128, 150, 152, 0, 175, 0, 10,
            0, 0, 0, 0, 0, 255, 63, 148, 82, 163, 7, 0, 0, 0, 0, 106, 101, 42, 103, 242, 124],
                   data);
    }

    let mut buf = [0; 20];
    unsafe {
        let mut encoder = Encoder::with_buf(&mut buf);
        println!("{:?}", encoder);
        encoder.uncheck_put_fixed32(2);
        encoder.uncheck_put_fixed64(655535);
        encoder.uncheck_put_fixed64(8_3980_4651_1103);
        println!("{:?}", &encoder);
        if let MutVector(data) = encoder.data {
            assert_eq!(&mut vec![0, 0, 0, 2, 0, 0, 0, 0, 0, 10, 0, 175, 0, 0, 7, 163, 82, 148, 63, 255],
                       data);
        }
    }


    let mut slice = Slice::from_vec(vec![0; 20]);
    unsafe {
        let mut encoder = Encoder::with_slice(&mut slice);
        println!("{:?}", encoder);
        encoder.uncheck_put_fixed32(2);
        encoder.uncheck_put_fixed64(655535);
        encoder.uncheck_put_fixed64(8_3980_4651_1103);
        println!("{:?}", &encoder);
        if let MutVector(data) = encoder.data {
            assert_eq!(&mut vec![0, 0, 0, 2, 0, 0, 0, 0, 0, 10, 0, 175, 0, 0, 7, 163, 82, 148, 63, 255],
                       data);
        }
    }

    Ok(())
}

#[test]
fn test_get_fixed() -> Result<()> {
    let mut vec = vec![];

    unsafe {
        let mut encoder = Encoder::with_vec(&mut vec);
        println!("{:?}", encoder);
        encoder.uncheck_put_fixed32(2);
        encoder.uncheck_put_fixed32(128);
        encoder.uncheck_put_fixed32(255);
        encoder.uncheck_put_fixed32(65535);
        encoder.uncheck_put_fixed32(10000000);
        encoder.uncheck_put_fixed64(655535);
        encoder.uncheck_put_fixed64(8_3980_4651_1103);
        encoder.uncheck_put_fixed64(900_3372_0368_5477_5808);
        println!("{:?}", &encoder.data);
        println!("{:?}", &encoder);
        if let MutVector(data) = encoder.data {
            assert_eq!(&mut vec![2, 0, 0, 0, 128, 0, 0, 0, 255, 0, 0, 0, 255, 255, 0, 0, 128, 150, 152, 0, 175, 0, 10, 0, 0, 0, 0, 0, 255, 63, 148, 82, 163, 7, 0, 0, 0, 0, 106, 101, 42, 103, 242, 124],
                       data);
        }
    }

    let mut decoder = Decoder::with_vec(&mut vec);

    while decoder.can_get() {
        let value = unsafe { decoder.uncheck_get_fixed32() };
        println!("{}", value);
    }
    let mut decoder = Decoder::with_vec(&mut vec);

    println!("can_get: {}", decoder.can_get());
    assert_eq!(true, decoder.can_get());

    assert_eq!(2, unsafe { decoder.uncheck_get_fixed32() });
    assert_eq!(128, unsafe { decoder.uncheck_get_fixed32() });
    assert_eq!(255, unsafe { decoder.uncheck_get_fixed32() });
    assert_eq!(65535, unsafe { decoder.uncheck_get_fixed32() });
    assert_eq!(10000000, unsafe { decoder.uncheck_get_fixed32() });
    assert_eq!(655535, unsafe { decoder.uncheck_get_fixed64() });
    assert_eq!(8_3980_4651_1103, unsafe { decoder.uncheck_get_fixed64() });
    assert_eq!(900_3372_0368_5477_5808, unsafe { decoder.uncheck_get_fixed64() });

    println!("can_get: {}", decoder.can_get());
    assert_eq!(false, decoder.can_get());

    let mut decoder = Decoder::with_vec(&mut vec);

    println!("can_get: {}", decoder.can_get());
    assert_eq!(true, decoder.can_get());

    assert_eq!(2, decoder.get_fixed32()?);
    assert_eq!(128, decoder.get_fixed32()?);
    assert_eq!(255, decoder.get_fixed32()?);
    assert_eq!(65535, decoder.get_fixed32()?);
    assert_eq!(10000000, decoder.get_fixed32()?);
    assert_eq!(655535, decoder.get_fixed64()?);
    assert_eq!(8_3980_4651_1103, decoder.get_fixed64()?);
    assert_eq!(900_3372_0368_5477_5808, decoder.get_fixed64()?);

    println!("{}", decoder.can_get());
    assert_eq!(false, decoder.can_get());

    Ok(())
}

#[test]
fn test_put_varint() -> Result<()> {
    let mut vec = vec![];
    unsafe {
        let mut encoder = Encoder::with_vec(&mut vec);
        encoder.uncheck_put_varint32(2);
        encoder.uncheck_put_varint32(128);
        encoder.uncheck_put_varint32(255);
        encoder.uncheck_put_varint32(65535);
        encoder.uncheck_put_varint32(10000000);
        encoder.uncheck_put_varint64(655535);
        encoder.uncheck_put_varint64(8_3980_4651_1103);
        encoder.uncheck_put_varint64(900_3372_0368_5477_5808);
        println!("{:?}", vec);
        assert_eq!(vec![2, 128, 1, 255, 1, 255, 255, 3, 128, 173, 226, 4, 175, 129, 40, 255, 255, 208, 148, 181, 244, 1, 128, 128, 168, 171, 166, 229, 153, 249, 124],
                   vec);
    }
    {
        let mut encoder = Encoder::with_vec(&mut vec);
        encoder.put_varint32(2)?;
        encoder.put_varint32(128)?;
        encoder.put_varint32(255)?;
        encoder.put_varint32(65535)?;
        encoder.put_varint32(10000000)?;
        encoder.put_varint64(655535)?;
        encoder.put_varint64(8_3980_4651_1103)?;
        encoder.put_varint64(900_3372_0368_5477_5808)?;
        println!("{:?}", vec);
        assert_eq!(vec![2, 128, 1, 255, 1, 255, 255, 3, 128, 173, 226, 4, 175, 129, 40, 255, 255, 208, 148, 181, 244, 1, 128, 128, 168, 171, 166, 229, 153, 249, 124],
                   vec);
    }
    Ok(())
}

#[test]
fn test_get_varint() -> Result<()> {
    let mut vec = vec![];
    unsafe {
        let mut encoder = Encoder::with_vec(&mut vec);
        encoder.uncheck_put_varint32(2);
        encoder.uncheck_put_varint32(128);
        encoder.uncheck_put_varint32(255);
        encoder.uncheck_put_varint32(65535);
        encoder.uncheck_put_varint32(10000000);
        encoder.uncheck_put_varint64(655535);
        encoder.uncheck_put_varint64(8_3980_4651_1103);
        encoder.uncheck_put_varint64(900_3372_0368_5477_5808);
        println!("{:?}", vec);
    };
    {
        let mut decoder = Decoder::with_vec(&mut vec);
        assert_eq!(2, decoder.get_varint32()?);
        assert_eq!(128, decoder.get_varint32()?);
        assert_eq!(255, decoder.get_varint32()?);
        assert_eq!(65535, decoder.get_varint32()?);
        assert_eq!(10000000, decoder.get_varint32()?);
        assert_eq!(655535, decoder.get_varint64()?);
        assert_eq!(8_3980_4651_1103, decoder.get_varint64()?);
        assert_eq!(900_3372_0368_5477_5808, decoder.get_varint64()?);
    };
    Ok(())
}

#[test]
fn test_put_buf() -> Result<()> {
    let mut vec = vec![];
    let mut encoder = Encoder::with_vec(&mut vec);
    let buf = [1, 2, 3];
    unsafe { encoder.uncheck_put_buf(&buf) }
    println!("{:?}", buf);
    encoder.put_buf(&buf)?;
    assert_eq!(&[1_u8, 2, 3, 1, 2, 3], vec.as_slice());
    println!("{:?}", vec);

    Ok(())
}

#[test]
fn test_get_buf() -> Result<()> {
    let mut vec = vec![];
    {
        let mut encoder = Encoder::with_vec(&mut vec);
        let buf = [1, 2, 3];
        unsafe { encoder.uncheck_put_buf(&buf) }
        println!("{:?}", buf);
        assert_eq!(&[1_u8, 2, 3], vec.clone().as_slice());
    }
    let mut decoder = Decoder::with_vec(&vec);
    let buf = unsafe { decoder.uncheck_get_buf(3) };
    println!("{:?}", buf);
    assert_eq!(Slice::from_vec(vec![1, 2, 3]), buf);
    assert_eq!(3, decoder.offset);

    Ok(())
}

#[test]
fn test_put_length_prefixed_slice() {
    let mut vec = vec![];
    {
        let mut encoder = Encoder::with_vec(&mut vec);
        let slice = Slice::from_vec(vec![1, 2, 3]);
        unsafe { encoder.uncheck_put_length_prefixed_slice(&slice); }
        assert_eq!(4, encoder.offset)
    }
    println!("{:?}", vec);
    assert_eq!(&vec![3, 1, 2, 3], &vec);
}

#[test]
fn test_get_length_prefixed_slice() {
    let mut vec = vec![];
    {
        let mut encoder = Encoder::with_vec(&mut vec);
        let slice = Slice::from_vec(vec![1, 2, 3]);
        unsafe { encoder.uncheck_put_length_prefixed_slice(&slice); }
    }
    println!("{:?}", vec);
    assert_eq!(vec![3, 1, 2, 3], vec);

    let mut decoder = Decoder::with_vec(&vec);
    let slice = unsafe { decoder.uncheck_get_length_prefixed_slice() };
    println!("{:?}", slice);
    assert_eq!(&[1_u8, 2, 3], &*slice);
    assert_eq!(4, decoder.offset)
}

#[test]
fn test_mixed_put_get() {
    let mut vec = vec![];
    let mut encoder = Encoder::with_vec(&mut vec);

    unsafe {
        encoder.uncheck_put_fixed32(3);
        encoder.uncheck_put_varint32(65535);
        encoder.uncheck_put_fixed64(7);
        encoder.uncheck_put_varint64(8_3980_4651_1103);
        let buf = [1, 2, 3];
        encoder.uncheck_put_buf(&buf);
        let slice = Slice::from_vec(vec![1, 2, 3]);
        encoder.uncheck_put_length_prefixed_slice(&slice);
    }

    let mut decoder = Decoder::with_vec(&vec);
    unsafe {
        assert_eq!(3, decoder.uncheck_get_fixed32());
        assert_eq!(65535, decoder.uncheck_get_varint32());
        assert_eq!(7, decoder.uncheck_get_fixed64());
        assert_eq!(8_3980_4651_1103, decoder.uncheck_get_varint64());
        let buf = [1_u8, 2, 3];
        assert_eq!(&buf, &*decoder.uncheck_get_buf(3));
        let slice = Slice::from_vec(vec![1, 2, 3]);
        assert_eq!(slice, decoder.uncheck_get_length_prefixed_slice())
    }
}

#[test]
fn test_offset_len_skip() -> Result<()> {
    let mut vec = vec![];
    let mut encoder = Encoder::with_vec(&mut vec);
    assert_eq!(0, encoder.offset());
    assert_eq!(0, encoder.len());
    encoder.put_varint32(65535)?;
    assert_eq!(3, encoder.offset());
    assert_eq!(3, encoder.len());

    encoder.put_varint32(65535)?;
    assert_eq!(6, encoder.offset());
    assert_eq!(6, encoder.len());

    encoder.put_varint32(65535)?;
    assert_eq!(9, encoder.offset());
    assert_eq!(9, encoder.len());

    let mut decoder = Decoder::with_vec(&vec);
    assert_eq!(0, decoder.offset());
    assert_eq!(9, decoder.limit());

    let value = decoder.get_varint32()?;
    assert_eq!(3, decoder.offset());
    assert_eq!(9, decoder.limit());
    assert_eq!(65535, value);

    decoder.skip(3)?;

    let value = decoder.get_varint32()?;
    assert_eq!(9, decoder.offset());
    assert_eq!(9, decoder.limit());
    assert_eq!(65535, value);

    let mut decoder = Decoder::with_vec(&vec);
    assert_eq!(0, decoder.offset());
    assert_eq!(9, decoder.limit());

    let value = decoder.get_varint32()?;
    assert_eq!(3, decoder.offset());
    assert_eq!(9, decoder.limit());
    assert_eq!(65535, value);

    unsafe { decoder.uncheck_skip(3); }

    let value = decoder.get_varint32()?;
    assert_eq!(9, decoder.offset());
    assert_eq!(9, decoder.limit());
    assert_eq!(65535, value);

    Ok(())
}

#[test]
fn test_from_into() {
    let mut data = vec![1, 2, 3];
    let encoder = Encoder::with_vec(&mut data);
    println!("{:?}", encoder);

    let decoder = encoder.create_decoder();
    println!("{:?}", decoder);
    assert_eq!(0, decoder.offset);
    let empty = &vec![];
    assert_eq!(vec![1, 2, 3], *if let Vector(data) = decoder.data { data } else { empty });
    assert_eq!(3, decoder.limit);
}

#[test]
fn test_type_capacity() {
    let type_capacity = type_capacity!(u32);
    println!("u32: {}", type_capacity);
    assert_eq!(4, type_capacity);
    let type_capacity = type_capacity!(u64);
    println!("u64: {}", type_capacity);
    assert_eq!(8, type_capacity);
}

#[test]
fn test_swap_bytes() {
    let value = 0x04030201_u32;
    let new_value = swap_bytes!(value);
    println!("value: {:?}, new_value: {:?}", value, new_value);
    assert_eq!(value, new_value);
    // 小端存储bytes
    let buf = [0x01, 0x02, 0x03, 0x04];
    let decode = unsafe { uncheck_decode_fixed32(&Buffer(&buf), 0) };
    // 小端存储的0x01,0x02,0x03,0x04解出来的数据要等于0x04030201_u32
    println!("value: {:?}, decode: {:?}", value, decode);
    assert_eq!(value, decode);
}