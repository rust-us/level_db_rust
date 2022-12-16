use crate::traits::filter_policy_trait::{FilterPolicy};
use crate::util::slice::Slice;

pub struct BloomFilterPolicy {
    bits_per_key: usize,
    k: usize
}

impl FilterPolicy for BloomFilterPolicy {

    fn name() -> String {
        String::from("leveldb.BuiltinBloomFilter2")
    }

    fn create_filter(&self, keys: Slice, n: u32, dst: String) -> String {
        todo!()
    }

    fn key_may_match(key: &Slice, filter: &Slice) -> bool {
        todo!()
    }
}