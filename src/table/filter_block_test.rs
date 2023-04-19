
mod test {
    use std::borrow::BorrowMut;
    use std::sync::Arc;
    use crate::debug;
    use crate::table::filter_block;
    use crate::table::filter_block::{FilterBlock, FilterBlockBuilder, FilterBlockReader};
    use crate::table::filter_block_test_filter_policy::TestHashFilter;
    use crate::traits::filter_policy_trait::FilterPolicy;
    use crate::util::slice::Slice;
    use crate::util::hash::{Hash, ToHash};

    use crate::util::Result;

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

        let filter_block_reader: FilterBlockReader = FilterBlockReader::new_with_policy(policy, &contents);

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
        let mut filter_block_builder: FilterBlockBuilder =
            FilterBlockBuilder::new_with_policy(policy.clone());

        // filter block 的 offset
        filter_block_builder.start_block(100);
        filter_block_builder.add_key_from_str("foo");
        filter_block_builder.add_key_from_str("bar");
        filter_block_builder.add_key_from_str("box");
        filter_block_builder.start_block(200);
        filter_block_builder.add_key_from_str("box");
        filter_block_builder.start_block(300);
        filter_block_builder.add_key_from_str("hello");

        let sliceRs: Result<Slice> = filter_block_builder.finish();
        debug!("sliceRs:{:?}", &sliceRs);

        let reader = FilterBlockReader::new_with_policy(
            policy.clone(), &sliceRs.unwrap());

        // todo  key_may_match  not impl
        // assert!(reader.key_may_match(100, &Slice::from("foo")));
        // assert!(reader.key_may_match(100, &Slice::from("bar")));
        // assert!(reader.key_may_match(100, &Slice::from("box")));
        // assert!(reader.key_may_match(100, &Slice::from("hello")));
        // assert!(reader.key_may_match(100, &Slice::from("foo")));
        // assert!(!reader.key_may_match(100, &Slice::from("missing")));
        // assert!(!reader.key_may_match(100, &Slice::from("other")));
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