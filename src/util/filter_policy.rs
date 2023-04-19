use crate::util::filter_policy_bloom::BloomFilterPolicy;
use crate::util::hash::{Hash, ToHash};
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