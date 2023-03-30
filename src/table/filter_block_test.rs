
mod test {
    use std::sync::Arc;
    use crate::table::filter_block;
    use crate::table::filter_block::{FilterBlock, FilterBlockBuilder, FilterBlockReader};
    use crate::traits::filter_policy_trait::FilterPolicy;
    use crate::util::filter_policy::BloomFilterPolicy;
    use crate::util::slice::Slice;

    #[test]
    fn test_filter_block_new_with_policy() {
        let policy = Arc::new(BloomFilterPolicy::new(2));

        let filter_block: FilterBlockBuilder<BloomFilterPolicy> = FilterBlockBuilder::new_with_policy(policy);

        let fp = filter_block.get_policy();
        let filter_policy_name = fp.name();
        assert_eq!(filter_policy_name, "leveldb.BuiltinBloomFilter");
        assert_eq!(filter_block.get_key(), "");
        assert_eq!(filter_block.get_result(), "");
        assert_eq!(filter_block.get_start().len(), 0);
        assert_eq!(filter_block.get_tmp_keys().len(), 0);
        assert_eq!(filter_block.get_tmp_filter_offsets().len(), 0);
    }

    #[test]
    fn test_filter_block_reader_new_with_policy_empty_content() {
        let policy = Arc::new(BloomFilterPolicy::new(2));
        let contents = Slice::default();

        let filter_block_reader: FilterBlockReader<BloomFilterPolicy> = FilterBlockReader::new_with_policy(policy, contents);

        let fp_reader = filter_block_reader.get_policy();
        let _reader_filter_policy_name = fp_reader.name();
        assert_eq!(_reader_filter_policy_name, "leveldb.BuiltinBloomFilter");
    }

    // #[test]
    // fn test_filter_block_reader_new_with_policy_with_content() {
    //     let policy = Arc::new(BloomFilterPolicy::new(2));
    //     let contents = Slice::default("");
    //
    //     let filter_block_reader: FilterBlockReader<BloomFilterPolicy> = FilterBlockReader::new_with_policy(policy, contents);
    //
    //     let fp_reader = filter_block_reader.get_policy();
    //     let _reader_filter_policy_name = fp_reader.name();
    //     assert_eq!(_reader_filter_policy_name, "leveldb.BuiltinBloomFilter");
    //     // assert_eq!(filter_block_reader.get_key(), "");
    // }
}