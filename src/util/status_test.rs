
mod test {
    use crate::util::slice::Slice;
    use crate::util::status::LevelError;

    #[test]
    fn test_of() {
        let msg1 = "abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc";
        let msg2 = "456456456456456456456456456456456456456456456456";

        let err: LevelError = LevelError::io_error(String::from(msg1).into(),
                                        String::from(msg2).into());
        let slice: Option<Slice> = err.into_msg();
        assert_eq!("abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc",
                   String::from(slice.unwrap()));

        let err: LevelError = LevelError::OK();
        let slice: Option<Slice> = err.into_msg();
        assert!(Option::None == slice);
    }

    // #[test]
    // fn test_toString() {
    //     // ok
    //     let status: LevelError = LevelError::OK();
    //     assert_eq!("OK", status.to_string());
    //
    //     let msg1 = "abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc\
    //     abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc\
    //     abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc\
    //     abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc\
    //     abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc";
    //     let msg2 = "456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456\
    //     456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456\
    //     456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456\
    //     456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456\
    //     456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456456";
    //
    //     let status: Status = Status::of(Code::kIOError,
    //                                     String::from(msg1).into(),
    //                                     String::from(msg2).into());
    //
    //     let expectString: String = String::from("".to_owned() + msg1 + "");
    //     assert_eq!("IO error:    101".to_owned() + msg1 + ": " + msg2,
    //                status.to_string());
    // }

}
