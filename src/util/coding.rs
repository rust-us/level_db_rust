use std::ops::Deref;
use crate::traits::coding_trait::CodingTrait;
use crate::traits::coding_trait::Coding32;
use crate::traits::coding_trait::Coding64;
use crate::util::slice::Slice;

macro_rules! varint {
    ($TYPE: ty, $NAME: ident, $SNAME: expr) => {
         fn $NAME(mut value: $TYPE, buf: &mut [u8], mut offset: usize) -> usize {
            const B: $ TYPE = 128;

            while value >= B {
                buf[offset] = (value | B) as u8;
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
    fn put_fixed32(mut dst: &mut String, value: u32) {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        Self::encode_fixed32(value, &mut buf, 0);
        for b in buf.iter() {
            dst.push(char::from(*b));
        }
    }

    fn put_fixed64(mut dst: &mut String, value: u64) {
        let mut buf: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        Self::encode_fixed64(value, &mut buf, 0);
        for b in buf.iter() {
            dst.push(char::from(*b));
        }
    }

    varint!(u32,encode_varint32);

    varint!(u64,encode_varint64);

    fn put_varint32(mut dst: &mut String, value: u32) {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        Self::encode_fixed32(value, &mut buf, 0);
        for b in buf.iter() {
            dst.push(char::from(*b));
        }
    }

    fn put_varint64(mut dst: &mut String, value: u64) {
        let mut buf: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        Self::encode_fixed64(value, &mut buf, 0);
        for b in buf.iter() {
            dst.push(char::from(*b));
        }
    }

    fn put_length_prefixed_slice(dst: &mut String, value: &mut Slice) {
        Self::put_varint64(dst, value.size() as u64);
        for b in value.borrow_data().as_bytes() {
            dst.push(char::from(*b));
        }
    }

    fn get_varint32(input: &mut Slice) -> u32 {
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

    fn get_varint64(input: &mut Slice) -> u64 {
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
        let len = 0_u32;
        let decode = Coding::get_varint32(input);
        Slice::from_buf(decode.to_le_bytes().as_mut_slice())
    }

    fn varint_length(mut value: u64) -> i32 {
        let mut len = 1;
        while value >= 128 {
            value >>= 7;
            len += 1;
        }
        len
    }

    fn encode_fixed32(mut value: u32, buf: &mut [u8], mut offset: usize) -> usize {
        buf[offset] = value as u8;
        offset += 1;
        buf[offset] = (value >> 8) as u8;
        offset += 1;
        buf[offset] = (value >> 16) as u8;
        offset += 1;
        buf[offset] = (value >> 24) as u8;
        offset += 1;
        offset
    }

    fn encode_fixed64(mut value: u64, buf: &mut [u8], mut offset: usize) -> usize {
        buf[offset] = value as u8;
        offset += 1;
        buf[offset] = (value >> 8) as u8;
        offset += 1;
        buf[offset] = (value >> 16) as u8;
        offset += 1;
        buf[offset] = (value >> 24) as u8;
        offset += 1;
        buf[offset] = (value >> 32) as u8;
        offset += 1;
        buf[offset] = (value >> 40) as u8;
        offset += 1;
        buf[offset] = (value >> 48) as u8;
        offset += 1;
        buf[offset] = (value >> 56) as u8;
        offset += 1;
        offset
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

macro_rules! encoding_impl {
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
            fn varint(self, buf: &mut [u8], mut offset: usize) -> usize {
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
            fn fixedint(self, buf: &mut [u8], mut offset: usize) -> usize {
                Coding::$FIXED_NAME (self, buf, offset)
            }
        }
    }
}

encoding_impl!(Coding32,u32,encode_varint32,encode_fixed32);

encoding_impl!(Coding64,u64,encode_varint64,encode_fixed64);