
mod test {
    use crate::util::r#const::COLON_WHITE_SPACE;
    use crate::util::slice::Slice;
    use crate::util::status::{LevelError, Status};

    #[test]
    fn test_wraper() {
        let msg1 = "abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc";

        let status = Status::wrapper(LevelError::KIOError, String::from(msg1).into());
        assert!(&status.is_io_error());
        let slice: Slice = status.into_msg();
        assert_eq!("abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc",
                   String::from(slice));

        let ss = Status::wrapper(LevelError::KOk, String::from(msg1).into());
        assert!(&ss.is_ok());
        assert_eq!("OK", &ss.to_string());
    }

    #[test]
    fn test_wrappers() {
        let msg1 = "abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc";
        let msg2 = "456456456456456456456456456456456456456456456456";

        let status = Status::wrappers(LevelError::KIOError, String::from(msg1).into(), String::from(msg2).into());
        let slice: Slice = status.into_msg();
        assert_eq!("abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc: 456456456456456456456456456456456456456456456456",
                   String::from(slice));

        let err: Status = LevelError::invalid_argument(String::from(msg1).into(),
                                                   String::from(msg2).into());
        assert!(&err.is_invalid_argument());
        // assert_eq!(err.into_code(), 4);

        let err: Status = LevelError::corruption(String::from(msg1).into(),
                                                   String::from(msg2).into());
        assert!(&err.is_corruption());
        // assert_eq!(err.into_code(), 2);

        let err: Status = LevelError::not_found(String::from(msg1).into(),
                                                   String::from(msg2).into());
        assert!(&err.is_not_found());

        let err: Status = LevelError::not_supported(String::from(msg1).into(),
                                                   String::from(msg2).into());
        assert!(&err.is_not_supported_error());

        let err: LevelError = LevelError::KOk;
        assert!(&err.is_ok());

        let err: LevelError = LevelError::default();
        assert!(&err.is_ok());
    }

    #[test]
    fn test_is_default() {
        let err: Status = LevelError::ok();
        assert!(err.is_ok());

        let err: Status = LevelError::io_error(String::from("a").into(),
                                               String::from("b").into());
        assert!(!err.is_ok());

        let status: Status = LevelError::not_found(String::from("a").into(),
                                                   String::from("b").into());
        assert!(status.is_not_found());
        assert!(status.get_error().is_not_found());
    }

    #[test]
    fn test_status_to_string() {
        // ok
        let status: Status = LevelError::ok();
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

        let error: Status = LevelError::invalid_argument(String::from(msg1).into(),
                                                      String::from(msg2).into());

        let binding = error.to_string();
        let error_msg = binding.as_str();
        println!("{}", error_msg);

        let expect_string: String = format!("Invalid argument: {}{}{}", String::from(msg1), COLON_WHITE_SPACE,
                                           String::from(msg2));
        assert_eq!(expect_string,  error_msg);
    }

    #[test]
    fn test_level_error_toString() {
        // ok
        let status: Status = LevelError::ok();
        assert_eq!("OK", status.to_string());

        // err invalid_argument
        let msg1 = "bcabcabcabcabcabcbc";
        let msg2 = "56";
        let error: Status = LevelError::invalid_argument(String::from(msg1).into(),
                                                      String::from(msg2).into());

        let le_err: LevelError = error.get_error();
        println!("{}", &le_err);

        // Display
        assert_eq!(String::from("Invalid argument: "), le_err.to_string());
    }

    #[test]
    fn test_level_error_try_from() -> Result<(), String> {
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
