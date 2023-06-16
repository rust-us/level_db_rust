
mod test {
    use crate::table::block_builder::BlockBuilder;
    use crate::util::options::OptionsPtr;

    use crate::util::Result;
    use crate::util::slice::Slice;

    #[test]
    fn test_new() {
        let opt = OptionsPtr::default();

        let block_builder = BlockBuilder::new(opt);
        assert_eq!(block_builder.get_restarts().len(), 1);
    }

    #[test]
    fn test_add() {
        let opt = OptionsPtr::default();

        let mut block_builder = BlockBuilder::new(opt);
        block_builder.add(Slice::from("a"), Slice::from("b"));
        assert_eq!(block_builder.get_restarts().len(), 1);
    }
}