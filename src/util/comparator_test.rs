
mod test {
    use std::cmp::Ordering;
    use crate::traits::comparator_trait::ComparatorTrait;
    use crate::util::comparator::{BytewiseComparatorImpl, InternalKeyComparator};
    use crate::util::slice::Slice;

    #[test]
    fn test_bytewise_comparator_impl_get_name() {
        let name = BytewiseComparatorImpl::get_name();
        println!("{}", &name);
        assert_eq!("leveldb.BytewiseComparator", name);
    }

    #[test]
    fn test_bytewise_comparator_impl_compare() {
        let comp = BytewiseComparatorImpl::default();
        let optionVal = comp.compare(&Slice::from("a"), &Slice::from("ab"));
        assert_eq!(optionVal.unwrap(), Ordering::Less);

        let comp = BytewiseComparatorImpl::default();
        let optionVal = comp.compare(&Slice::from("b"), &Slice::from("abcd"));
        assert_eq!(optionVal.unwrap(), Ordering::Greater);

        let comp = BytewiseComparatorImpl::default();
        let optionVal = comp.compare(&Slice::from("abcd"), &Slice::from("abcd"));
        assert_eq!(optionVal.unwrap(), Ordering::Equal);
    }

}