
mod test {
    use crate::db::version_edit;
    use crate::db::version_edit::{Tag, VersionEdit};
    use crate::util::slice::Slice;

    #[test]
    fn test_Tag() {
        let tag = Tag::kCompactPointer;
        assert_eq!(tag.get_value(), 5);

        let tag1 = Tag::k_comparator;
        let v = tag1.get_value();
        assert_eq!(v, 1);
    }

    #[test]
    fn test_version_edit_encode_to() {
        let mut target: Vec<u8> = vec![];

        let version_edit = VersionEdit::new_with_log_number(6);
        version_edit.encode_to(&mut target);
        println!("target: {}.", &target.len());
        // todo
        // assert_eq!(target.len(), 2);
    }

    #[test]
    fn test_version_edit_decode_from() {
        let source = Slice::from("a");

        let mut version_edit = VersionEdit::new();
        let status = version_edit.decode_from(&source);
        println!("status: {}.", status.get_error());
        // todo
        // assert_eq!(target.len(), 2);
    }
}