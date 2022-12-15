
mod test {
    use std::cmp::Ordering;
    use crate::traits::comparator_trait::ComparatorTrait;
    use crate::util::comparator::{BytewiseComparatorImpl};
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
        println!("find_short_successor_val: {}", &find_short_successor_val);
        assert_eq!(find_short_successor_val, "a");
    }

}