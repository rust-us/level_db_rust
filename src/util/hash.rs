use std::ops::{BitXor, Mul};
use std::mem::size_of;
use std::slice as stds;

use crate::traits::coding_trait::CodingTrait;
use crate::util::coding::Coding;
use crate::util::crc::AsCrc;
use crate::util::r#const::HASH_DEFAULT_SEED;
use crate::util::slice;
use crate::util::slice::Slice;

/// 一种可以计算 hash 的特质
pub trait ToHash {
    fn to_hash(&self) -> u32;
}

/// 所有基本类型 u8, i8, u16, u32 ... 的Vec都可以实现 hash 值计算
/// Sample:
/// ```
/// let hash = vec!['a','b','c'].to_hash();
/// ```
impl<T: Sized> ToHash for Vec<T> {
    fn to_hash(&self) -> u32 {
        let v_v = self.as_slice();

        v_v.to_hash()
    }
}

/// 所有基本类型 u8, i8, u16, u32 ... 的slice都可以实现 hash 值计算
/// Sample:
/// ```
/// let buf = ['a','b','c'];
/// let hash_val = &buf.as_slice().to_hash();
/// ```
impl<T: Sized> ToHash for &[T] {
    #[inline]
    fn to_hash(&self) -> u32 {
        let ptr_u8 = self.as_ptr() as *const _ as *const u8;

        let data = unsafe {
            stds::from_raw_parts(ptr_u8, size_of::<T>() * self.len())
        };

        Hash::hash_code(data, HASH_DEFAULT_SEED)
    }
}

/// 实现了 &str 转 ToHash 的特质
/// Sample:
/// ```
/// let hash = "abc".to_hash();
/// ```
impl ToHash for &str {
    fn to_hash(&self) -> u32 {
        Hash::hash_code(self.as_bytes(), HASH_DEFAULT_SEED)
    }
}

/// 实现了 Slice 转 ToHash 的特质
/// Sample:
/// ```
///     let val = "aabbccd";
///     let slice: Slice = Slice::from_buf(val.as_bytes());
///     let slice_hash_val = slice.to_hash();
/// ```
impl ToHash for Slice {
    fn to_hash(&self) -> u32 {
        Hash::hash_code(self.to_vec().as_slice(), HASH_DEFAULT_SEED)
    }
}

/// 实现了 String 转 ToHash 的特质
/// Sample:
/// ```
///     let val = "aabbccd";
///     let val_s = String::from(val);
///     let string_hash_val = val_s.to_hash();
/// ```
impl ToHash for String {
    fn to_hash(&self) -> u32 {
        Hash::hash_code(self.as_bytes(), HASH_DEFAULT_SEED)
    }
}

/// 本方案中，采用的是MurMurHash的一种变体，是一种高效低碰撞的非加密型哈希函数。具有较高的平衡性与低碰撞率
pub struct Hash {}

impl Hash {
    pub fn hash_code(data: &[u8], seed: u32) -> u32 {
        let murmur_hash: u32 = 0xc6a4a793;
        let r: u32 = 24;

        let limit: usize = data.len();
        let mul_first = limit.mul(murmur_hash as usize); // x = data_size * murmur_hash
        let mut h: u32 = seed.bitxor(mul_first as u32);  // h = seed ^ x

        // 每次按照四字节长度读取字节流中的数据 w，并使用普通的哈希函数计算哈希值。
        let mut position: usize = 0;
        while position + 4 <= limit {
            //每次解码前4个字节，直到最后剩下小于4个字节
            // rust的 &[u8] 是胖指针，带长度信息的，会做range check，所以是安全的。
            // 虽然decode_fixed32 中也是解码4字节，但传入整个data在方法上不明确，因此传 [position..(position + 4)], 可以更加方便理解，对性能无影响
            let w = Coding::decode_fixed32(&data[position..(position + 4)]);

            // 向后移动4个字节
            position += 4;

            // /计算过程中使用了自然溢出特性
            // h += w
            h = h.wrapping_add(w);
            // h *= m
            h = h.wrapping_mul(murmur_hash);
            // ^ 按位异或 bitxor , >> 右移位 shr, << 左移位 shl
            // h ^= (h >> 16) == h ^= h.shr(16);
            h = h.bitxor(h.wrapping_shr(16));
        }

        // 四字节读取则为了加速，最终可能剩下 3/2/1 个多余的字节，
        // 将剩下的字节转化到 h 里面
        let mut mark: usize = 0;
        while limit - position - mark != 0 {
            match limit - position - mark {
                3 => {
                    let as_us: u32 = data[position + 2] as u32;
                    h = h.wrapping_add(as_us.wrapping_shl(16));

                    mark += 1;
                }
                2 => {
                    let as_us: u32 = data[position + 1] as u32;
                    h = h.wrapping_add(as_us.wrapping_shl(8));

                    mark += 1;
                }
                1 => {
                    let as_us: u32 = data[position] as u32;
                    h = h.wrapping_add(as_us);
                    // h *= m
                    h = h.wrapping_mul(murmur_hash);
                    // h ^= (h >> r) ==> h ^= h.shr(r);
                    h = h.bitxor(h.wrapping_shr(r));

                    mark += 1;
                }
                _ => {
                    println!("0")
                }
            };
        }

        h
    }
}