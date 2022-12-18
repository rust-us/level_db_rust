
mod test {
    use std::cmp::Ordering;
    use std::io::Write;
    use crate::traits::comparator_trait::ComparatorTrait;
    use crate::util::comparator::{BytewiseComparatorImpl, InternalKeyComparator};
    use crate::util::slice::Slice;

    #[test]
    fn test_bytewise_comparator_impl_get_name() {
        let name = BytewiseComparatorImpl::get_name();
        println!("get_name: {}", &name);
        assert_eq!("leveldb.BytewiseComparator", name);
    }

    #[test]
    fn test_bytewise_comparator_impl_compare() {
        let comp = BytewiseComparatorImpl::default();
        let option_val = comp.compare(&Slice::from("a"), &Slice::from("ab"));
        assert_eq!(option_val.unwrap(), Ordering::Less);

        // todo  Slice 存在 bug 未修复
        // let comp = BytewiseComparatorImpl::default();
        // let option_val = comp.compare(&Slice::from("b"), &Slice::from("abcd"));
        // assert_eq!(option_val.unwrap(), Ordering::Greater);

        let comp = BytewiseComparatorImpl::default();
        let option_val = comp.compare(&Slice::from("abcd"), &Slice::from("abcd"));
        assert_eq!(option_val.unwrap(), Ordering::Equal);
    }

    #[test]
    fn test_bytewise_comparator_impl_find_shortest_separator() {
        let comp = BytewiseComparatorImpl::default();
        let find_shortest_separator_val = comp.find_shortest_separator(
            &String::from("helloWorld"),
            &Slice::from("helloZookeeper"));
        // W < Z
        assert_eq!(find_shortest_separator_val, "helloX");

        let comp = BytewiseComparatorImpl::default();
        let find_shortest_separator_val = comp.find_shortest_separator(
            &String::from("abcdefghijklimx"),
            &Slice::from("abcdefghijklimNy"));
        // x(!X) > N
        assert_eq!(find_shortest_separator_val, "abcdefghijklimx");

        let comp = BytewiseComparatorImpl::default();
        let find_shortest_separator_val = comp.find_shortest_separator(
            &String::from("abcdefghijklimA"),
            &Slice::from("abcdefghijklimNy"));
        // A < N
        assert_eq!(find_shortest_separator_val, "abcdefghijklimB");

        let comp = BytewiseComparatorImpl::default();
        let find_shortest_separator_val = comp.find_shortest_separator(
            &String::from("abcdefghijklima"),
            &Slice::from("abcdefghijklimNy"));
        // a > N
        assert_eq!(find_shortest_separator_val, "abcdefghijklima");

        let comp = BytewiseComparatorImpl::default();
        let find_shortest_separator_val = comp.find_shortest_separator(
            &String::from("abcdefghijklima"),
            &Slice::from("abcdefghijklimny"));
        // a < n
        assert_eq!(find_shortest_separator_val, "abcdefghijklimb");
    }

    #[test]
    fn test_bytewise_comparator_impl_find_short_successor() {
        let comp = BytewiseComparatorImpl::default();
        let find_short_successor_val = comp.find_short_successor(&String::from("helloWorld"));
        assert_eq!(find_short_successor_val, "i");


        let comp = BytewiseComparatorImpl::default();
        let find_short_successor_val = comp.find_short_successor(&String::from("a"));
        assert_eq!(find_short_successor_val, "b");


        let comp = BytewiseComparatorImpl::default();
        let find_short_successor_val = comp.find_short_successor(&String::from("123"));
        assert_eq!(find_short_successor_val, "2");


        // 只有 u8::MAX
        let u8_max_vec: Vec<u8> = vec![u8::MAX];
        let u8_max_str = String::from(Slice::from_buf(u8_max_vec.as_slice()));

        let comp = BytewiseComparatorImpl::default();
        let find_short_successor_val = comp.find_short_successor(&u8_max_str);
        assert_eq!(u8_max_str, find_short_successor_val);


        // u8max 结尾
        let mut u8_vec: Vec<u8> = vec![];
        u8_vec.write(&String::from("helloWorld").as_bytes().to_vec());
        u8_vec.push(u8::MAX);

        let u8_array_str = String::from(Slice::from_buf(u8_vec.as_slice()));

        let comp = BytewiseComparatorImpl::default();
        let find_short_successor_val = comp.find_short_successor(&u8_array_str);
        assert_eq!(find_short_successor_val, "i");


        // u8max 开头
        let mut u8_vec: Vec<u8> = vec![];
        u8_vec.push(u8::MAX);
        u8_vec.write(&String::from("helloWorld").as_bytes().to_vec());
        let u8_max_str = String::from(Slice::from_buf(u8_vec.as_slice()));

        let comp = BytewiseComparatorImpl::default();
        let find_short_successor_val = comp.find_short_successor(&u8_max_str);

        // 只有 u8::MAX
        let mut expect_u8_max_vec: Vec<u8> = vec![];
        expect_u8_max_vec.push(u8::MAX);
        expect_u8_max_vec.write("i".as_bytes()).expect("panic message");
        assert_eq!(find_short_successor_val,
                   String::from(Slice::from_buf(expect_u8_max_vec.as_slice())));
    }

    #[test]
    fn test_internal_key_comparator_get_name() {
        let name = InternalKeyComparator::get_name();
        assert_eq!("leveldb.InternalKeyComparator", name);
    }

}