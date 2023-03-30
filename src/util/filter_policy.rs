use std::ops::{BitOr, Mul, Shl};
use crate::traits::filter_policy_trait::{FilterPolicy};
use crate::util::hash::{Hash, ToHash};
use crate::util::r#const::HASH_DEFAULT_SEED;
use crate::util::slice::Slice;

pub trait FromPolicy {
    fn from_bits_per_key(&self) -> usize;

    fn from_k(&self) -> usize;
}

/// 其他成员的语义扩展
pub trait AsBloomHash {
    #[inline]
    fn bloom_hash(&self) -> u32;
}

/// 实现了 Slice 转 bloom_hash 的特质
/// Sample:
/// ```
///     use rand::distributions::Slice;
/// let val = "aabbccd";
///     let slice = Slice::from_buf(val.as_bytes());
///     let hash_val = slice.bloom_hash();
/// ```
impl AsBloomHash for Slice {
    #[inline]
    fn bloom_hash(&self) -> u32 {
        BloomFilterPolicy::bloom_hash(self)
    }
}

// #########################  BloomFilterPolicy
pub struct BloomFilterPolicy {
    bits_per_key: usize,
    k: usize
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

impl<'a> BloomFilterPolicy {
    pub fn bloom_hash(key: &Slice) -> u32 {
        key.to_hash_with_seed(HASH_DEFAULT_SEED)
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

// dyn FilterPolicy + FromPolicy
impl FilterPolicy for BloomFilterPolicy {

    fn name(&self) -> String {
        String::from("leveldb.BuiltinBloomFilter")
    }

    fn create_filter(&self, keys: Vec<Slice>) -> Slice {
        let n: usize = keys.len();

        let mut bits: usize = n * self.bits_per_key;

        // For small n, we can see a very high false positive rate.
        // Fix it by enforcing a minimum bloom filter length.
        if bits < 64 {
            bits = 64;
        }

        let bytes: usize = (bits + 7) / 8;
        bits = bytes * 8;

        let mut dst_chars: Vec<u8> = vec![0; bytes + 1];
        dst_chars[bytes] = self.k as u8;

        for i in 0..n {
            let slice = keys.get(i).unwrap();

            let mut h : u32 = slice.bloom_hash();
            let delta : u32 = (h >> 17) | (h << 15);

            for j in 0..self.k {
                let bitpos:usize = ((h as usize) % bits);

                // a |= b  -->  按位或， 后赋值给a
                let position: usize = bitpos / 8;
                let mod_val: usize = bitpos % 8;
                let val = (1 as u8).wrapping_shl(mod_val as u32);

                dst_chars[position] |= val;

                h = h.wrapping_add(delta);
            }
        }

        // Vec<u8> 转 Slice
        Slice::from_buf(&dst_chars)
    }

    fn key_may_match(&self, key: &Slice, bloom_filter: &Slice) -> bool {
        let filter_size: usize = bloom_filter.size();
        if filter_size < 2 {
            return false;
        }

        let bloom_filter_array:Vec<u8>  = bloom_filter.to_vec();
        let bits: usize = (filter_size - 1) * 8;

        // Use the encoded k so that we can read filters generated by bloom filters created using different parameters.
        let k: u8 = bloom_filter_array[filter_size - 1];
        if k > 30 {
            // Reserved for potentially new encodings for short bloom filters.  Consider it a match.
            return true;
        }

        let mut h : u32 = key.bloom_hash();
        // Rotate right 17 bits
        let delta = (h >> 17) | (h << 15);

        for j in 0..k {
            let bitpos:usize = ((h as usize) % bits);
            if (bloom_filter_array[bitpos/8] & (1 << (bitpos % 8))) == 0 {
                return false;
            }

            h = h.wrapping_add(delta);
        }

        return true;
    }
}

// #########################  InternalFilterPolicy
pub struct InternalFilterPolicy {
    user_policy_: dyn FilterPolicy
}

impl InternalFilterPolicy {
    fn new(policy: Box<dyn FilterPolicy>) -> Box<InternalFilterPolicy> {
        // InternalFilterPolicy{ user_policy_: policy }
        todo!()
    }
}

impl FilterPolicy for InternalFilterPolicy {
    fn name(&self) -> String {
        todo!()
    }

    fn create_filter(&self, keys: Vec<Slice>) -> Slice {
        // 根据指定的参数创建过滤器，并返回结果， 结果为dst的原始内容 + append结果。
        // 参数keys[0,n-1]包含依据用户提供的comparator排序的key列表--可重复，
        // 并把根据这些key创建的filter追加到 dst中。
        //
        todo!()
    }

    fn key_may_match(&self, key: &Slice, bloom_filter: &Slice) -> bool {
        todo!()
    }

}