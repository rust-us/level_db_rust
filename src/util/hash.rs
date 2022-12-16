use std::ops::{BitXor, Mul};
use crate::traits::coding_trait::CodingTrait;
use crate::util::coding::Coding;

/// 本方案中，采用的是MurMurHash的一种变体，是一种高效低碰撞的非加密型哈希函数。具有较高的平衡性与低碰撞率
pub struct Hash {}

impl<'a> Hash {
    ///
    ///
    /// # Arguments
    ///
    /// * `data`:
    /// * `n`: data 的长度
    /// * `seed`:  随机数种子
    ///
    /// returns: u32
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn hash(mut data: String, data_size: usize, seed: u32) -> u32 {
        let data_u8_vec;
        unsafe {
            data_u8_vec = data.as_mut_vec();
        }

        Hash::hash_char(data_u8_vec, data_size, seed)
    }

    pub fn hash_char(data: &Vec<u8>, data_size: usize, seed: u32) -> u32 {
        let murmur_hash : u32 = 0xc6a4a793;
        let r : u32 = 24;

        let limit: usize = data_size;
        let mul_first = data_size.mul(murmur_hash as usize); // x = data_size * murmur_hash
        let mut h: usize = seed.bitxor(mul_first as u32) as usize;  // h = seed ^ x

        // 每次按照四字节长度读取字节流中的数据 w，并使用普通的哈希函数计算哈希值。
        let mut position: usize = 0;
        while position + 4 <= limit {
            //每次解码前4个字节，直到最后剩下小于4个字节
            // rust的 &[u8] 是胖指针，带长度信息的，会做range check，所以是安全的。
            let slice_str: &[u8] = data[position..(position + 4)].as_ref();
            let w: u32 = Coding::decode_fixed32(slice_str);

            // 向后移动4个字节
            position += 4;

            // /计算过程中使用了自然溢出特性
            // h += w
            h = h.wrapping_add(w as usize);
            // h *= m
            h = h.wrapping_mul(murmur_hash as usize);
            // ^ 按位异或 bitxor , >> 右移位 shr, << 左移位 shl
            // h ^= (h >> 16) == h ^= h.shr(16);
            h = h.bitxor(h.wrapping_shr(16));
        }

        // 四字节读取则为了加速，最终可能剩下 3/2/1 个多余的字节，
        // 将剩下的字节转化到 h 里面
        let cu = limit - position;
        match cu {
            3 => {
                let us: &[u8] = data[position..].as_ref();
                h = h.wrapping_add((us[2] as u32).wrapping_shl(16) as usize);
                h = h.wrapping_add((us[1] as u32).wrapping_shl(8) as usize);
                h = h.wrapping_add(us[0].into());
            },
            2 => {
                let us: &[u8] = data[position..].as_ref();
                h = h.wrapping_add((us[1] as u32).wrapping_shl(8) as usize);
                h = h.wrapping_add(us[0].into());
            },
            1 => {
                let us: &[u8] = data[position..].as_ref();
                h = h.wrapping_add(us[0].into());
                // h *= m
                h = h.wrapping_mul(murmur_hash as usize);
                // h ^= (h >> r) == h ^= h.shr(r);
                h = h.bitxor(h.wrapping_shr(r));
            },
            _ => {}
        };

        println!("hash usize: {}", h);
        println!("hash u32: {}", h as u32);

        h as u32
    }
}