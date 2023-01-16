use std::ops::{BitOr, Mul, Shl};
use crate::traits::filter_policy_trait::{FilterPolicy};
use crate::util::hash::{Hash, ToHash};
use crate::util::slice::Slice;

pub trait FromPolicy {
    fn from_bits_per_key(&self) -> usize;
    fn from_k(&self) -> usize;
}

pub struct BloomFilterPolicy {
    bits_per_key: usize,
    k: usize
}

impl<'a> BloomFilterPolicy {
    pub fn bloom_hash(key: &Slice) -> u32 {
        key.to_hash_with_seed(0xbc9f1d34)
    }
}

/// get struct  BloomFilterPolicy 属性
impl FromPolicy for BloomFilterPolicy {
    fn from_bits_per_key(&self) -> usize {
        self.bits_per_key
    }

    fn from_k(&self) -> usize {
        self.k
    }
}

impl BloomFilterPolicy {
    pub fn new(bits_per_key: usize) -> Self {
        // We intentionally round down to reduce probing cost a little bit
        // 0.69 =~ ln(2)
        let factor: f64 = 0.69;
        let mut k_k: usize = factor.mul(bits_per_key as f64).round() as usize;

        if k_k < 1 {
            k_k = 1;
        }
        if k_k > 30{
            k_k = 30;
        }

        Self {
            bits_per_key,
            k : k_k
        }
    }
}

// dyn FilterPolicy + FromPolicy
impl FilterPolicy for BloomFilterPolicy {

    fn name(&self) -> String {
        String::from("leveldb.BuiltinBloomFilter2")
    }

    fn create_filter(&self, keys: Vec<Slice>, n: usize) -> String {
        // 根据指定的参数创建过滤器，并返回结果。
        // 参数keys[0,n-1]包含依据用户提供的comparator排序的key列表--可重复，
        // 并把根据这些key创建的filter追加到 返回结果中。
        let mut bits: usize = n * self.bits_per_key;

        // For small n, we can see a very high false positive rate.  Fix it
        // by enforcing a minimum bloom filter length.
        if bits < 64 {
            bits = 64;
        }

        let bytes: usize = (bits + 7) / 8;
        bits = bytes * 8;

        let mut dstChars: Vec<u8> = Vec::with_capacity(bytes);
        for i in 0..n {
            let mut h : u32 = BloomFilterPolicy::bloom_hash(keys.get(i).unwrap());
            let delta : u32 = (h >> 17) | (h << 15);

            for j in 0..self.k {
                let bitpos:usize = ((h as usize) % bits);

                // a |= b  -->  按位或， 后赋值给a
                let position: usize = bitpos / 8;
                let mod_val: usize = bitpos % 8;
                let val = (1 as u8).wrapping_shl(mod_val as u32);

                // TODO  error: index out of bounds: the len is 0 but the index is 161
                dstChars[position] |= val;

                h = h.wrapping_add(delta);
            }
        }

        // Vec<u8> 转 String
        let ss = Slice::from_buf(&dstChars).borrow_data();

        ss.to_string()
    }

    fn key_may_match(&self, key: &Slice, filter: &Slice) -> bool {
        todo!()
    }
}