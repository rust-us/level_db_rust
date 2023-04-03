
mod test {
    use std::borrow::BorrowMut;
    use std::sync::Arc;
    use crate::table::filter_block;
    use crate::table::filter_block::{FilterBlock, FilterBlockBuilder, FilterBlockReader};
    use crate::traits::coding_trait::CodingTrait;
    use crate::traits::filter_policy_trait::FilterPolicy;
    use crate::util::coding::Coding;
    use crate::util::filter_policy::BloomFilterPolicy;
    use crate::util::slice::Slice;
    use crate::util::hash::{Hash, ToHash};

    use crate::util::Result;

    pub struct TestHashFilter {
        //.
    }

    impl TestHashFilter {
        fn new() -> Self {
            Self {

            }
        }
    }

    impl FilterPolicy for TestHashFilter {
        fn name(&self) -> String {
            String::from("TestHashFilter")
        }

        fn create_filter(&self, keys: Vec<&Slice>) -> Slice {
            let mut n: usize = 0;
            for i in 0..keys.len() {
                n += keys[i].len();
            }

            self.create_filter_with_len(n, keys)
        }

        fn create_filter_with_len(&self, len: usize, keys: Vec<&Slice>) -> Slice {
            let mut n: usize = len;

            let mut dst_chars = vec![0; n];
            let dst_chars_u8 = dst_chars.borrow_mut();

            let mut offset: usize = 0;
            for i in 0..keys.len() {
                let h = Hash::hash_code(keys[i].as_ref(), 1);
                let of = Coding::put_fixed32(dst_chars_u8, offset, h);
                offset += of;
            }

            Slice::from_buf(dst_chars_u8)
        }

        fn key_may_match(&self, key: &Slice, bloom_filter: &Slice) -> bool {
            let h = Hash::hash_code(key.to_vec().as_slice(), 1);

            let mut pos = 0;
            while pos <= bloom_filter.size() {
                let buf = &bloom_filter.as_ref()[pos..];

                if h == Coding::decode_fixed32(buf) {
                    return true
                }

                pos += 4;
            }

            false
        }
    }

    // #[test]
    // fn test_create_filter() {
    //     let policy = TestHashFilter::new();
    //
    //     let s1 = Slice::try_from(String::from("hello")).unwrap();
    //     let s2 = Slice::try_from(String::from("world")).unwrap();
    //     let mut keys : Vec<&Slice>  = Vec::new();
    //     keys.push(&s1);
    //     keys.push(&s2);
    //
    //     let bloom_filter: Slice = policy.create_filter(keys);
    //
    //     let mut key_may_match = policy.key_may_match(
    //         &Slice::try_from(String::from("hello")).unwrap(),
    //         &bloom_filter);
    //     assert!(key_may_match);
    //
    //     key_may_match = policy.key_may_match(&Slice::try_from(String::from("world")).unwrap(),
    //                                          &bloom_filter);
    //     assert!(key_may_match);
    //
    //     let mut key_not_match = policy.key_may_match(&Slice::try_from(String::from("x")).unwrap(),
    //                                                  &bloom_filter);
    //     assert!(!key_not_match);
    //
    //     key_not_match = policy.key_may_match(&Slice::try_from(String::from("helloworld")).unwrap(),
    //                                          &bloom_filter);
    //     assert!(!key_not_match);
    //
    //     key_not_match = policy.key_may_match(&Slice::try_from(String::from("hello world")).unwrap(),
    //                                          &bloom_filter);
    //     assert!(!key_not_match);
    //
    //     key_not_match = policy.key_may_match(&Slice::try_from(String::from("foo")).unwrap(),
    //                                          &bloom_filter);
    //     assert!(!key_not_match);
    // }

    #[test]
    fn test_filter_block_new_with_policy() {
        let policy: Arc<Box<dyn FilterPolicy>> = Arc::new(Box::new(TestHashFilter::new()));

        let filter_block: FilterBlockBuilder = FilterBlockBuilder::
                                            new_with_policy_capacity(policy, 10);

        let fp = filter_block.get_policy();
        let filter_policy_name = fp.name();
        assert_eq!(filter_policy_name, "TestHashFilter");
        assert_eq!(filter_block.get_keys().len(), 0);
        assert_eq!(filter_block.get_result().len(), 0);
        assert_eq!(filter_block.get_start().len(), 0);
        assert_eq!(filter_block.get_tmp_keys().len(), 0);
        assert_eq!(filter_block.get_tmp_filter_offsets().len(), 0);
    }

    #[test]
    fn test_filter_block_reader_new_with_policy_empty_content() {
        let policy: Arc<Box<dyn FilterPolicy>> = Arc::new(Box::new(TestHashFilter::new()));
        let contents = Slice::default();

        let filter_block_reader: FilterBlockReader = FilterBlockReader::new_with_policy(policy, contents);

        let fp_reader = filter_block_reader.get_policy();
        let _reader_filter_policy_name = fp_reader.name();
        assert_eq!(_reader_filter_policy_name, "TestHashFilter");
        assert_eq!(filter_block_reader.get_data().len(), 0);
        assert_eq!(filter_block_reader.get_offset().len(), 0);
        assert_eq!(filter_block_reader.get_num(), 0);
        assert_eq!(filter_block_reader.get_base_lg(), 0);
    }

    #[test]
    fn test_filter_block_new_with_policy_and_addkey() {
        let policy: Arc<Box<dyn FilterPolicy>> = Arc::new(Box::new(TestHashFilter::new()));
        let mut filter_block_builder: FilterBlockBuilder = FilterBlockBuilder::new_with_policy_capacity(
            policy, 10);

        filter_block_builder.start_block(100);
        filter_block_builder.add_key_from_str("foo");
        filter_block_builder.add_key_from_str("bar");
        filter_block_builder.add_key_from_str("box");
        filter_block_builder.start_block(200);
        filter_block_builder.add_key_from_str("box");
        filter_block_builder.start_block(300);
        filter_block_builder.add_key_from_str("hello");

        let sliceRs: Result<Slice> = filter_block_builder.finish();
        assert_eq!("a", "leveldb.BuiltinBloomFilter");
    }

    // #[test]
    // fn test_filter_block_reader_new_with_policy_with_content() {
    //     let policy = Arc::new(BloomFilterPolicy::new(2));
    //     let contents = Slice::from("\000");
    //
    //     let filter_block_reader: FilterBlockReader<BloomFilterPolicy> = FilterBlockReader::new_with_policy(policy, contents);
    //
    //     let fp_reader = filter_block_reader.get_policy();
    //     let _reader_filter_policy_name = fp_reader.name();
    //     assert_eq!(_reader_filter_policy_name, "leveldb.BuiltinBloomFilter");
    //     assert_eq!(filter_block_reader.get_data().len(), 0);
    //     assert_eq!(filter_block_reader.get_offset().len(), 0);
    //     assert_eq!(filter_block_reader.get_num(), 0);
    //     assert_eq!(filter_block_reader.get_base_lg(), 0);
    // }
}