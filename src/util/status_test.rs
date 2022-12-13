
mod test {
    use crate::traits::status_trait::StatusTrait;
    use crate::util::r#const::COLON_WHITE_SPACE;
    use crate::util::slice::Slice;
    use crate::util::status::LevelError;

    #[test]
    fn test_error_code() {
        let msg1 = "abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc";
        let msg2 = "456456456456456456456456456456456456456456456456";

        let err: LevelError = LevelError::io_error(String::from(msg1).into(), String::from(msg2).into());
        assert!(&err.is_io_error());
        // assert!(&err.into_code() == 5);
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
        // assert_eq!(err.into_code(), 4);

        let err: LevelError = LevelError::corruption(String::from(msg1).into(),
                                                   String::from(msg2).into());
        assert!(&err.is_corruption());
        // assert_eq!(err.into_code(), 2);

        let err: LevelError = LevelError::not_found(String::from(msg1).into(),
                                                   String::from(msg2).into());
        assert!(&err.is_not_found());
        // assert_eq!(err.into_code(), 1);

        let err: LevelError = LevelError::not_supported(String::from(msg1).into(),
                                                   String::from(msg2).into());
        assert!(&err.is_not_supported_error());
        // assert_eq!(err.into_code(), 3);

        let err: LevelError = LevelError::default();
        assert!(&err.is_ok());
        // assert_eq!(err.into_code(), 0);
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
        let error_msg = binding.as_str();
        println!("{}", error_msg);

        let expect_string: String = format!("Invalid argument: {}{}{}", String::from(msg1), COLON_WHITE_SPACE,
                                           String::from(msg2));
        assert_eq!(expect_string,  error_msg);
    }

    #[test]
    fn test_is_default() {
        let err: LevelError = LevelError::ok();
        assert!(err.is_default());

        let err: LevelError = LevelError::io_error(String::from("a").into(),
                                                   String::from("b").into());
        assert!(!err.is_default());

        // let err: LevelError = LevelError::ok();
        // let a = err.into();
        // print!("{}", a);
    }

    #[test]
    fn test_try_from() -> Result<(), String> {
        let rs = LevelError::try_from(1)?;
        assert!(&rs.is_not_found());
        let rs: Result<LevelError, String> = 1.try_into();
        assert!(rs.ok().unwrap().is_not_found());

        let rs = LevelError::try_from(0)?;
        assert!(&rs.is_ok());
        let rs: Result<LevelError, String> = 0.try_into();
        assert!(rs.ok().unwrap().is_ok());

        let rs = LevelError::try_from(2)?;
        assert!(&rs.is_corruption());
        let rs: LevelError = 2.try_into()?;
        assert!(rs.is_corruption());

        let rs: LevelError = LevelError::try_from(3)?;
        assert!(&rs.is_not_supported_error());
        let rs: LevelError = 3.try_into()?;
        assert!(rs.is_not_supported_error());

        let rs = LevelError::try_from(4)?;
        assert!(&rs.is_invalid_argument());

        let rs = LevelError::try_from(5)?;
        assert!(&rs.is_io_error());

        let rs = LevelError::try_from(6);
        assert_eq!("Unknown code: 6", rs.err().unwrap());

        Ok(())
    }

}
