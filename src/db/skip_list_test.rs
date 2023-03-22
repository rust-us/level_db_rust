mod test {
    use std::collections::HashSet;
    use std::sync::{Arc, Mutex};
    use rand::Rng;
    use crate::db::DefaultSkipList;
    use crate::debug;
    use crate::util::Arena;
    use crate::util::comparator::BytewiseComparatorImpl;
    use crate::util::Result;
    use crate::util::slice::Slice;
    use crate::util::unsafe_slice::TryIntoUnsafeSlice;

    #[test]
    fn test_add() -> Result<()> {
        let cmp = Arc::new(BytewiseComparatorImpl::default());
        let arena = Arc::new(Mutex::new(Arena::default()));
        let mut list = DefaultSkipList::create(cmp, arena.clone());
        let len = 10;
        for i in 0..len {
            list.insert(format!("key_{}", i).try_into_unsafe_slice(arena.clone())?).expect("insert ok");
        }
        assert_eq!(10, list.len(), "expect 10, but actually is: {}", list.len());
        debug!("{}", list.to_string());
        for i in 0..len {
            let key: Slice = format!("key_{}", i).into();
            debug!("contains key: {}", key);
            assert!(list.contains(&key), "contains key: {}", key);
        }
        list.iter().for_each(|slice| {
            debug!("slice: {}", slice.as_str())
        });
        Ok(())
    }

    #[test]
    fn test_rnd_add() -> Result<()> {
        let cmp = Arc::new(BytewiseComparatorImpl::default());
        let arena = Arc::new(Mutex::new(Arena::default()));
        let mut list = DefaultSkipList::create(cmp, arena.clone());
        let len = 10;
        let mut rnd = rand::thread_rng();
        let mut set = HashSet::new();
        for _i in 0..10 {
            let j = rnd.gen_range(0..len);
            let key = format!("key_{}", j);
            set.insert(key.clone());
            list.insert(key.try_into_unsafe_slice(arena.clone())?)?;
            debug!("skiplist: {}", list.to_string());
        }
        assert_eq!(set.len(), list.len(), "list length must eq: {}", list.len());
        set.iter().for_each(|key| {
            let c = list.contains(&key);
            assert!(c, "must contains key: {}", key)
        });

        Ok(())
    }
}