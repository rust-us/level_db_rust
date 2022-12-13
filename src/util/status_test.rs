
mod test {
    use crate::util::r#const::COLON_WHITE_SPACE;
    use crate::util::slice::Slice;
    use crate::util::status::LevelError;

    #[test]
    fn test_error_code() {
        let msg1 = "abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc";
        let msg2 = "456456456456456456456456456456456456456456456456";

        let err: LevelError = LevelError::io_error(String::from(msg1).into(),
                                        String::from(msg2).into());
        assert!(&err.is_io_error());
        // assert!(&err.get_code() == 5);
        let slice: Option<Slice> = err.into_msg();
        assert_eq!("abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc: 456456456456456456456456456456456456456456456456",
                   String::from(slice.unwrap()));

        let err: LevelError = LevelError::ok();
        assert!(&err.is_ok());
        let slice: Option<Slice> = err.into_msg();
        assert!(Option::None == slice);

        let err: LevelError = LevelError::invalid_argument(String::from(msg1).into(),
                                                   String::from(msg2).into());
        assert!(&err.is_invalid_argument());
        // assert_eq!(err.get_code(), 4);

        let err: LevelError = LevelError::corruption(String::from(msg1).into(),
                                                   String::from(msg2).into());
        assert!(&err.is_corruption());
        // assert_eq!(err.get_code(), 2);

        let err: LevelError = LevelError::not_found(String::from(msg1).into(),
                                                   String::from(msg2).into());
        assert!(&err.is_not_found());
        // assert_eq!(err.get_code(), 1);

        let err: LevelError = LevelError::not_supported(String::from(msg1).into(),
                                                   String::from(msg2).into());
        assert!(&err.is_not_supported_error());
        // assert_eq!(err.get_code(), 3);

        let err: LevelError = LevelError::default();
        assert!(&err.is_ok());
        // assert_eq!(err.get_code(), 0);
    }

    #[test]
    fn test_toString() {
        // ok
        let status: LevelError = LevelError::ok();
        assert_eq!("OK", status.to_string());

        let msg1 = "abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc\
        abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc\
        abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc\
        abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc\
        abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc";
        let msg2 = "456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456\
        456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456\
        456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456\
        456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456\
        456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456";

        let error: LevelError = LevelError::invalid_argument(String::from(msg1).into(),
                                                      String::from(msg2).into());

        let binding = error.to_string();
        let errorMsg = binding.as_str();
        println!("{}", errorMsg);

        let expectString: String = format!("Invalid argument: {}{}{}", String::from(msg1), COLON_WHITE_SPACE,
                                           String::from(msg2));
        assert_eq!(expectString,  errorMsg);
    }

}
