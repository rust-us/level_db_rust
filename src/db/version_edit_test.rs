
mod test {
    use crate::db::version_edit;
    use crate::db::version_edit::{Tag, VersionEdit};
    use crate::util::slice::Slice;
    use crate::util::Result;

    #[test]
    fn test_tag() {
        let tag = Tag::kCompactPointer;
        assert_eq!(tag.get_value(), 5);

        let tag1 = Tag::k_comparator;
        let v = tag1.get_value();
        assert_eq!(v, 1);
    }

    #[test]
    fn test_version_edit_encode_to() -> Result<()> {
        let mut target: Vec<u8> = vec![];

        let version_edit = VersionEdit::new_with_log_number(6);
        version_edit.encode_to(&mut target)?;
        println!("target: {}.", &target.len());
        // todo
        // assert_eq!(target.len(), 2);

        Ok(())
    }

    #[test]
    fn test_version_edit_decode_from_default() {
        let source = Slice::from("a");

        let mut version_edit = VersionEdit::new();
        let status = version_edit.decode_from(&source);
        assert!(&status.is_corruption());
        assert_eq!(&status.get_msg(), "VersionEdit: unknown tag");
    }

    #[test]
    fn test_version_edit_decode_from() {
        let source = Slice::from("a");

        let mut version_edit = VersionEdit::new_with_log_number(6);
        let status = version_edit.decode_from(&source);
        assert!(&status.is_corruption());
        assert_eq!(&status.get_msg(), "VersionEdit: unknown tag");
    }

    #[test]
    fn test_version_edit_debug_string() {
        let mut version_edit = VersionEdit::new_with_log_number(6);
        let debug_str = version_edit.debug_string();
        println!("debug_str: \n {}", debug_str);
    }
}