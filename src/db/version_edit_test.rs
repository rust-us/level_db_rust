
mod test {
    use crate::db::version_edit;
    use crate::db::version_edit::{Tag, VersionEdit};

    #[test]
    fn test_Tag() {
        let tag = Tag::kCompactPointer;
        assert_eq!(tag.get_value(), 5);

        let tag1 = Tag::k_comparator;
        let v = tag1.get_value();
        assert_eq!(v, 1);


    }

    #[test]
    fn test_Version_edit() {
        let mut target: Vec<u8> = vec![];

        let version_edit = VersionEdit::new_with_log_number(6);
        version_edit.encode_to(&mut target);
        println!("target: {}.", &target.len());
        assert_eq!(target.len(), 2);
    }
}