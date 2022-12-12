mod test {
    use std::cmp::Ordering;
    use crate::util::slice::Slice;

    #[test]
    fn test_from() {
        // from &str
        let a0 = Slice::from("123");
        assert_eq!(String::from("123"), String::from(a0));
        // from String
        let a1 = Slice::from(String::from("123"));
        assert_eq!(String::from("123"), String::from(a1));

        let a2 = Slice::from_buf([30_u8, 31, 32].as_mut_slice());
        assert_eq!(String::from("012"), String::from(a2));
    }

    #[test]
    fn test_empty() {
        let a0 = Slice::default();
        assert_eq!(true, a0.empty());

        let a1 = Slice::from("123");
        assert_eq!(false, a1.empty());
    }

    #[test]
    fn test_remove_prefix() {
        let a0 = Slice::from("123");
        let a1 = a0.remove_prefix(1);
        assert_eq!(2, a1.len());
    }

    #[test]
    fn test_starts_with() {
        let a0 = Slice::from("12345");
        let a1 = a0.remove_prefix(2);
        assert_eq!(String::from("345"), String::from(a1));
    }

    #[test]
    fn test_borrow_data() {
        let mut a0 = Slice::from("123");
        let borrowed = a0.borrow_data();
        assert_eq!(3, borrowed.len());
        let owned = borrowed.to_owned();
        assert_eq!(3, owned.len());
    }

    #[test]
    fn test_partial_eq() {
        let a0 = Slice::from("123");
        let a1 = Slice::from("123");
        let a2 = Slice::from("234");
        let a3 = Slice::from("012");
        assert_eq!(true, a0 == a1);
        assert_eq!(true, a0 < a2);
        assert_eq!(true, a0 > a3);
    }

    #[test]
    fn test_partial_ord() {
        let a0 = Slice::from("123");
        let a1 = Slice::from("123");
        let a2 = Slice::from("234");
        let a3 = Slice::from("012");
        assert_eq!(Ordering::Equal, a0.partial_cmp(&a1).unwrap());
        assert_eq!(Ordering::Less, a0.partial_cmp(&a2).unwrap());
        assert_eq!(Ordering::Greater, a0.partial_cmp(&a3).unwrap());
    }

    #[test]
    fn test_memory_leak() {
        // 申请 100G 内存, 查看是否内存泄漏。如果内存泄漏，程序会OOM
        (0..100_000_000).for_each(|_| {
            // 1k
            let str = "0123012301230123012301230123012301230123012301230123012301230123012\
            301230123012301230123012301230123012301230123012301230123012301230123012301230123012\
            301230123012301230123012301230123012301230123012301230123012301230123012301230123012\
            301230123012301230123012301230123012301230123012301230123012301230123012301230123012\
            301230123012301230123012301230123012301230123012301230123012301230123012301230123012\
            301230123012301230123012301230123012301230123012301230123012301230123012301230123012\
            301230123012301230123012301230123012301230123012301230123012301230123012301230123012\
            301230123012301230123012301230123012301230123012301230123012301230123012301230123012\
            301230123012301230123012301230123012301230123012301230123012301230123012301230123012\
            301230123012301230123012301230123012301230123012301230123012301230123012301230123012\
            301230123012301230123012301230123012301230123012301230123012301230123012301230123012\
            301230123012301230123012301230123012301230123012301230123012301230123012301230123012\
            301230123012301230123012301230123";
            let _: Slice = str.into();
        })
    }
}