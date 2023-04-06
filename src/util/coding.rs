use std::io::Read;
use crate::traits::coding_trait::CodingTrait;
use crate::traits::coding_trait::Coding32;
use crate::traits::coding_trait::Coding64;
use crate::util::slice::Slice;

pub struct Coding {}

impl CodingTrait for Coding {
    fn put_fixed32(dst: &mut [u8], mut offset: usize, value: u32) -> usize {
        Self::encode_fixed32(value, dst, offset);
        offset += 4;
        offset
    }

    fn put_fixed64(dst: &mut [u8], mut offset: usize, value: u64) -> usize {
        Self::encode_fixed64(value, dst, offset);
        offset += 8;
        offset
    }

    fn encode_varint32(mut value: u32, buf: &mut [u8], mut offset: usize) -> usize {
        while value >= 128 {
            buf[offset] = (value | 128) as u8;
            value >>= 7;
            offset += 1;
        }
        buf[offset] = value as u8;
        offset += 1;
        offset
    }

    fn encode_varint64(mut value: u64, buf: &mut [u8], mut offset: usize) -> usize {
        while value >= 128 {
            buf[offset] = (value | 128) as u8;
            value >>= 7;
            offset += 1;
        }
        buf[offset] = value as u8;
        offset += 1;
        offset
    }

    fn put_varint32(dst: &mut [u8], mut offset: usize, value: u32) -> usize {
        Self::encode_varint32(value, dst, offset)
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

    fn get_varint32(input: &Slice, mut offset: usize) -> Option<(u32, usize)> {
        let bytes = &input[offset..input.size()];
        let mut shift = 0_u32;
        let limit = input.size();
        let mut i = 0;
        let mut value = 0_u32;
        while shift <= 28 && i < limit {
            let byte = bytes[i];
            i += 1;
            if (byte & 128) != 0 {
                value |= ((byte & 127) << shift) as u32;
                offset += 1;
            } else {
                // 溢出左移
                value |= (byte as u32) << shift;
                offset += 1;
                return Some((value, offset));
            }
            shift += 7;
        }
        None
    }

    fn get_varint64(input: &Slice, mut offset: usize) -> Option<(u64, usize)> {
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
        None
    }

    fn get_length_prefixed_slice(input: &mut Slice) -> Option<Slice> {
        let decode = Coding::get_varint32(input, 0);
        match decode {
            None => {
                None
            }
            Some(v) => {
                Some(Slice::from_buf(v.0.to_le_bytes().as_mut_slice()))
            }
        }
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
        buf[offset + 0] = value as u8;
        buf[offset + 1] = (value >> 8) as u8;
        buf[offset + 2] = (value >> 16) as u8;
        buf[offset + 3] = (value >> 24) as u8;
        offset += 4;
        offset
    }

    fn encode_fixed64(value: u64, buf: &mut [u8], mut offset: usize) -> usize {
        buf[offset + 0] = value as u8;
        buf[offset + 1] = (value >> 8) as u8;
        buf[offset + 2] = (value >> 16) as u8;
        buf[offset + 3] = (value >> 24) as u8;
        buf[offset + 4] = (value >> 32) as u8;
        buf[offset + 5] = (value >> 40) as u8;
        buf[offset + 6] = (value >> 48) as u8;
        buf[offset + 7] = (value >> 56) as u8;
        offset += 8;
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