use std::io::Write;
use crate::traits::coding_trait::CodingTrait;
use crate::traits::coding_trait::Coding32;
use crate::traits::coding_trait::Coding64;
use crate::util::slice::Slice;

macro_rules! varint {
    ($TYPE: ty, $NAME: ident, $SNAME: expr) => {
         fn $NAME(mut value: $TYPE, buf: &mut [u8], mut offset: usize) -> usize {
            while value >= 128 {
                buf[offset] = (value | 128) as u8;
                offset += 1;
                value >>= 7;
            }
            buf[offset] = value as u8;

            offset
        }
    };

    ($TYPE: ty, $NAME: ident) => {
        varint!( $TYPE, $NAME, stringify!($NAME));
    }
}

pub struct Coding {}

impl CodingTrait for Coding {
    fn put_fixed32(dst: &mut [u8], mut offset: usize, value: u32) -> usize {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        Self::encode_fixed32(value, &mut buf, 0);
        dst[offset] = buf[0];
        offset += 1;
        dst[offset] = buf[1];
        offset += 1;
        dst[offset] = buf[2];
        offset += 1;
        dst[offset] = buf[3];
        offset
    }

    fn put_fixed64(dst: &mut [u8], mut offset: usize, value: u64) -> usize {
        let mut buf: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        Self::encode_fixed64(value, &mut buf, 0);
        dst[offset] = buf[0];
        offset += 1;
        dst[offset] = buf[1];
        offset += 1;
        dst[offset] = buf[2];
        offset += 1;
        dst[offset] = buf[3];
        offset += 1;
        dst[offset] = buf[4];
        offset += 1;
        dst[offset] = buf[5];
        offset += 1;
        dst[offset] = buf[6];
        offset += 1;
        dst[offset] = buf[7];
        offset
    }

    varint!(u32,encode_varint32);

    varint!(u64,encode_varint64);

    fn put_varint32(dst: &mut [u8], mut offset: usize, value: u32) -> usize {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        let var_offset = Self::encode_varint32(value, &mut buf, 0);
        for i in 0..var_offset {
            dst[offset] = buf[i];
            offset += 1;
        }
        offset
    }

    fn put_varint64(dst: &mut [u8], mut offset: usize, value: u64) -> usize {
        let mut buf: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let var_offset = Self::encode_varint64(value, &mut buf, 0);
        for i in 0..var_offset {
            dst[offset] = buf[i];
            offset += 1;
        }
        offset
    }

    // fn put_length_prefixed_slice(dst: &mut [u8], offset: usize, value: &Slice) -> usize {
    fn put_length_prefixed_slice(dst: &mut [u8], offset: usize, value_len: usize) -> usize {
        Self::put_varint64(dst, offset, value_len as u64);
        offset
    }

    fn get_varint32(input: &Slice) -> u32 {
        let cow = input.borrow_data();
        let bytes = cow.as_bytes();
        let mut result = 0_u32;
        let mut shift = 0_u32;
        let limit = input.size();
        let mut i = 0;
        while shift <= 28 && i < limit {
            let b = bytes[i];
            i += 1;
            if (b & 128) != 0 {
                result |= ((b & 127) << shift) as u32;
            } else {
                result |= (b << shift) as u32;
            }
            shift += 7;
        }
        result
    }

    fn get_varint64(input: &Slice) -> u64 {
        let cow = input.borrow_data();
        let bytes = cow.as_bytes();
        let mut result = 0_u64;
        let mut shift = 0_u64;
        let limit = input.size();
        let mut i = 0;
        while shift <= 63 && i < limit {
            let b = bytes[i];
            i += 1;
            if (b & 128) != 0 {
                result |= ((b & 127) << shift) as u64;
            } else {
                result |= (b << shift) as u64;
            }
            shift += 7;
        }
        result
    }

    fn get_length_prefixed_slice(input: &mut Slice) -> Slice {
        let decode = Coding::get_varint32(input);
        Slice::from_buf(decode.to_le_bytes().as_mut_slice())
    }

    fn varint_length(mut value: usize) -> usize {
        let mut len = 1;
        while value >= 128 {
            value >>= 7;
            len += 1;
        }
        len
    }

    fn encode_fixed32(value: u32, buf: &mut [u8], mut offset: usize) -> usize {
        (&mut buf[offset..]).write(&value.to_le_bytes()).unwrap();
        offset+4
    }

    fn encode_fixed64(value: u64, buf: &mut [u8], mut offset: usize) -> usize {
        (&mut buf[offset..]).write(&value.to_le_bytes()).unwrap();
        offset+8
    }


    fn decode_fixed32(buf: &[u8]) -> u32 {
        return (buf[0] as u32) |
            (buf[1] as u32) << 8 |
            (buf[2] as u32) << 16 |
            (buf[3] as u32) << 24;
    }

    fn decode_fixed64(buf: &[u8]) -> u64 {
        return (buf[0]) as u64 |
            (buf[1] as u64) << 8 |
            (buf[2] as u64) << 16 |
            (buf[3] as u64) << 24 |
            (buf[4] as u64) << 32 |
            (buf[5] as u64) << 40 |
            (buf[6] as u64) << 48 |
            (buf[7] as u64) << 56;
    }
}

macro_rules! coding_impl {
    {$TRAIT: ident, $TYPE: ty, $VAR_NAME: ident, $FIXED_NAME: ident} => {
        impl $TRAIT for $TYPE {
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
            fn varint(self, buf: &mut [u8], offset: usize) -> usize {
                Coding::$VAR_NAME (self, buf, offset)
            }
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
            fn fixedint(self, buf: &mut [u8], offset: usize) -> usize {
                Coding::$FIXED_NAME (self, buf, offset)
            }
        }
    }
}

coding_impl!(Coding32,u32,encode_varint32,encode_fixed32);

coding_impl!(Coding64,u64,encode_varint64,encode_fixed64);