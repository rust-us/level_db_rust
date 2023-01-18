
mod test {
    use std::cmp::Ordering;
    use crate::db::file_meta_data::FileMetaData;

    #[test]
    fn test_eq() {
        let meta1: FileMetaData = FileMetaData::default();
        let meta2: FileMetaData = FileMetaData::default();
        let meta3: FileMetaData = FileMetaData::create_refs(6);
        let meta3_1: FileMetaData = FileMetaData::create_refs(6);

        assert!(meta1.eq(&meta2));
        assert!(!meta1.eq(&meta3));
        assert!(meta3.eq(&meta3_1));
    }

    #[test]
    fn test_partial_ord() {
        let meta0: FileMetaData = FileMetaData::default();
        let meta0_1: FileMetaData = FileMetaData::default();
        let meta3: FileMetaData = FileMetaData::create_refs(3);
        let meta6: FileMetaData = FileMetaData::create_refs(6);
        let meta7: FileMetaData = FileMetaData::create_refs(7);

        assert!(meta0.partial_cmp(&meta0_1).is_some());
    }

}