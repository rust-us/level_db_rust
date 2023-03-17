use crate::util::slice::Slice;

pub trait CodingTrait {
    ///32位定长编码写入字符串
    ///
    /// # Arguments
    ///
    /// * `dst`:  目标字符串
    /// * `value`: 编码值
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///    let mut string = String::from("encode:");
    ///    put_fixed32(&mut string, 65535);
    /// ```
    fn put_fixed32(dst: &mut [u8], offset: usize, value: u32) -> usize;
    ///64位定长编码写入字符串
    ///
    /// # Arguments
    ///
    /// * `dst`: 目标字符串
    /// * `value`: 编码值
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///    let mut string = String::from("encode:");
    ///    put_fixed64(&mut string, 65535);
    /// ```
    fn put_fixed64(dst: &mut [u8], offset: usize, value: u64) -> usize;
    /// 32位变长编码写入字符串
    ///
    /// # Arguments
    ///
    /// * `dst`: 目标字符串
    /// * `value`: 编码值
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///    let mut string = String::from("encode:");
    ///    put_varint32(&mut string, 65535);
    /// ```
    fn put_varint32(dst: &mut [u8], offset: usize, value: u32) -> usize;
    /// 64位变长编码写入字符串
    ///
    /// # Arguments
    ///
    /// * `dst`: 目标字符串
    /// * `value`: 编码值
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///    let mut string = String::from("encode:");
    ///    put_varint64(&mut string, 65535);
    /// ```
    fn put_varint64(dst: &mut [u8], offset: usize, value: u64) -> usize;
    /// 将slice的长度写入目标字符串
    ///
    /// # Arguments
    ///
    /// * `dst`: 目标字符串
    /// * `value`: Slice类型的编码值
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn put_length_prefixed_slice(dst: &mut [u8], offset: usize, value: Slice) -> usize;
    /// 从slice的开头解码一个32位的变长整数, 并将slice的索引置于解码后的位置
    ///
    /// # Arguments
    ///
    /// * `input`: slice
    ///
    /// returns: u32
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn get_varint32(input: &mut Slice) -> u32;
    /// 从slice的开头解码一个64位的变长整数, 并将slice的索引置于解码后的位置
    ///
    /// # Arguments
    ///
    /// * `input`: slice
    ///
    /// returns: u32
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn get_varint64(input: &mut Slice) -> u64;
    /// 从slice数据中读取长度 返回长度的Slice
    ///
    /// # Arguments
    ///
    /// * `input`: 输入数据
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn get_length_prefixed_slice(input: &mut Slice) -> Slice;
    /// 32位变长正整数编码
    ///
    /// # Arguments
    ///
    /// * `value`: 编码值
    /// * `buf`: 目标数组
    /// * `offset`: 偏移量
    ///
    /// returns: usize : 编码后的偏移量
    ///
    /// # Examples
    ///
    /// ```
    ///     let mut buf: [u8; 4] = [0, 0, 0, 0];
    ///     let value: u32 = 65534;
    ///     let offset =encode_varint32(value, &mut buf, 0);
    /// ```
    fn encode_varint32(value: u32, buf: &mut [u8], offset: usize) -> usize;
    /// 变长正整数编码
    ///
    /// # Arguments
    ///
    /// * `value`: 编码值
    /// * `buf`: 目标数组
    /// * `offset`: 偏移量
    ///
    /// returns: usize : 编码后的偏移量
    ///
    /// # Examples
    ///
    /// ```
    ///     let mut buf: [u8; 4] = [0, 0, 0, 0];
    ///     let value: u32 = 65534;
    ///     let offset =encode_varint64(value, &mut buf, 0);
    /// ```
    fn encode_varint64(value: u64, buf: &mut [u8], offset: usize) -> usize;
    /// 获取变长编码后的长度
    ///
    /// # Arguments
    ///
    /// * `value`: 编码值
    ///
    /// returns: i32
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    /// 从slice的开头解码一个32位的变长整数, 并将slice的索引置于解码后的位置
    fn varint_length(value: usize) -> usize;
    /// 32位定长正整数编码
    ///
    /// # Arguments
    ///
    /// * `value`: 编码值
    /// * `buf`: 目标数组
    /// * `offset`: 偏移量
    ///
    /// returns: usize : 编码后的偏移量
    ///
    /// # Examples
    ///
    /// ```
    ///     let mut buf: [u8; 4] = [0, 0, 0, 0];
    ///     let value: u32 = 65534;
    ///     let offset = Self::encode_fixed32(value, &mut buf, 0);
    /// ```
    fn encode_fixed32(value: u32, buf: &mut [u8], offset: usize) -> usize;
    /// 64位定长正整数编码
    ///
    /// # Arguments
    ///
    /// * `value`:
    /// * `buf`:
    /// * `offset`:
    ///
    /// returns: usize
    ///
    /// # Examples
    ///
    /// ```
    ///     let mut buf: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    ///     let value: u64 = 65534;
    ///     let offset = encode_fixed64(value, &mut buf, 0);
    /// ```
    fn encode_fixed64(value: u64, buf: &mut [u8], offset: usize) -> usize;
    /// 32位定长解码
    ///
    /// # Arguments
    ///
    /// * `buf`: 待解码数据
    ///
    /// returns: u32
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn decode_fixed32(buf: &[u8]) -> u32;
    /// 64位定长解码
    ///
    /// # Arguments
    ///
    /// * `buf`: 待解码数据
    ///
    /// returns: u64
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn decode_fixed64(buf: &[u8]) -> u64;
}

macro_rules! coding_trait {
    {$TRAIT: ident, $TYPE: ty} => {
        pub trait $ TRAIT {
            /// 变长正整数编码
            ///
            /// # Arguments
            ///
            /// * `buf`: 目标数组
            /// * `offset`: 偏移量
            ///
            /// returns: usize : 编码后的偏移量
            ///
            /// # Examples
            ///
            /// ```
            ///     let mut buf: [u8; 4] = [0, 0, 0, 0];
            ///     let value: u32 = 65534;
            ///     let offset = value.varint(&mut buf, 0);
            /// ```
            fn varint(self, buf: &mut [u8], offset: usize) -> usize;
            /// 定长正整数编码
            ///
            /// # Arguments
            ///
            /// * `buf`: 目标数组
            /// * `offset`: 偏移量
            ///
            /// returns: usize : 编码后的偏移量
            ///
            /// # Examples
            ///
            /// ```
            ///     let mut buf: [u8; 4] = [0, 0, 0, 0];
            ///     let value: u32 = 65534;
            ///     let offset = value.fixedint(&mut buf, 0);
            /// ```
            fn fixedint(self, buf: &mut [u8], offset: usize) -> usize;
        }
    }
}

coding_trait!(Coding32,u32);

coding_trait!(Coding64,u64);

