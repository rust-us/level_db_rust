
mod test {
    use std::collections::HashSet;
    use std::io::{stdout, Write};
    use std::panic;
    use std::sync::{Arc, Mutex};
    use rand::Rng;
    use crate::util::Result;
    use crate::db::DefaultSkipList;
    use crate::db::skip_list::SkipList;
    use crate::debug;
    use crate::util::Arena;
    use crate::util::comparator::BytewiseComparatorImpl;
    use crate::util::slice::Slice;

    #[test]
    fn test_add() -> Result<()> {
        let cmp = Arc::new(BytewiseComparatorImpl::default());
        let arena = Arc::new(Mutex::new(Arena::default()));
        let mut list = DefaultSkipList::create(cmp, arena);
        let len = 10;
        for i in 0..len {
            list.insert(format!("key_{}", i).into()).expect("insert ok");
        }
        assert_eq!(10, list.len(), "expect 10, but actually is: {}", list.len());
        println!("{}", list.to_string());
        for i in 0..len {
            let key: Slice = format!("key_{}", i).into();
            println!("contains key: {}", key);
            assert!(list.contains(&key), "contains key: {}", key);
        }
        list.iter().for_each(|slice| {
            println!("slice: {}", slice.as_str())
        });
        Ok(())
    }

    #[test]
    fn test_rnd_add() -> Result<()> {
        panic::set_hook(Box::new(|_panic_info| {
            stdout().flush().unwrap();
        }));
        let cmp = Arc::new(BytewiseComparatorImpl::default());
        let arena = Arc::new(Mutex::new(Arena::default()));
        let mut list = DefaultSkipList::create(cmp, arena);
        let len = 10;
        let mut rnd = rand::thread_rng();
        let mut set = HashSet::new();
        for i in 0..10 {
            let j = rnd.gen_range(0..len);
            let key = format!("key_{}", j);
            set.insert(key.clone());
            list.insert(key.into())?;
            debug!("skiplist: {}", list.to_string());
        }
        assert_eq!(set.len(), list.len(), "list length must eq: {}", list.len());
        set.iter().for_each(|key| {
            let c = list.contains(&key.clone().into());
            assert!(c, "must contains key: {}", key)
        });

        Ok(())
    }

}