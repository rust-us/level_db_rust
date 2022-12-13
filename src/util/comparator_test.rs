
mod test {
    use crate::traits::comparator_trait::ComparatorTrait;
    use crate::util::comparator::{BytewiseComparatorImpl, InternalKeyComparator};

    #[test]
    fn test() {
        let name = BytewiseComparatorImpl::get_name();
        println!("{}", &name);
        assert_eq!("leveldb.BytewiseComparator", name);
    }
}