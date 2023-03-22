mod test {
    use std::borrow::Borrow;
    use std::collections::HashMap;
    use std::ops::Deref;
    use crate::util::cache::{LRUHandle, ShardLRUCache};
    use crate::util::slice::Slice;

    use crate::util::Result;

    #[test]
    fn test_insert() -> Result<()> {
        let mut cache: ShardLRUCache<i32> = ShardLRUCache::new_with_capacity(16);
        let key = Slice::from("123");
        let value = 1234;
        cache.insert(key.clone(), value, 1, move |k, v| {
            println!("delete key: {}", String::from(k));
            println!("delete value: {}", v);
        })?;
        println!("key: {}", String::from(key.clone()));
        println!("value: {}", value);
        Ok(())
    }

    #[test]
    fn test_update() -> Result<()> {
        let mut cache: ShardLRUCache<i32> = ShardLRUCache::new_with_capacity(16);
        let key = Slice::from("123");
        let value = 1234;
        cache.insert(key.clone(), value, 1, move |k, v| {
            println!("delete key: {}", String::from(k));
            println!("delete value: {}", v);
        })?;
        println!("key: {}", String::from(key.clone()));
        println!("value: {}", value);
        let mut inserted = cache.lookup(&key.clone())?;
        assert_eq!(value, *inserted.unwrap().value());

        let value = 1235;
        cache.insert(key.clone(), value, 1, move |k, v| {
            println!("delete key: {}", String::from(k));
            println!("delete value: {}", v);
        })?;
        let mut inserted = cache.lookup(&key.clone())?;
        println!("key: {}", String::from(key.clone()));
        println!("value: {}", value);
        assert_eq!(value, *inserted.unwrap().value());

        Ok(())
    }

    #[test]
    fn test_lookup() -> Result<()> {
        let mut cache: ShardLRUCache<i32> = ShardLRUCache::new_with_capacity(16);
        let key = Slice::from("123");
        let value = 1234;
        cache.insert(key.clone(), value, 1, move |k, v| {
            println!("delete key: {}", String::from(k));
            println!("delete value: {}", v);
        })?;
        println!("key: {}", String::from(key.clone()));
        println!("value: {}", value);

        let value = cache.lookup(&key.clone())?;
        match value {
            None => {
                println!("value is none");
            }
            Some(v) => {
                println!("key: {}", String::from(v.key()));
                println!("value: {}", v.value());
            }
        }

        Ok(())
    }

    #[test]
    fn test_remove() -> Result<()> {
        let mut cache: ShardLRUCache<i32> = ShardLRUCache::new_with_capacity(16);
        let key = Slice::from("123");
        let value = 1234;
        cache.insert("123", value, 1, move |k, v| {
            println!("delete key: {}", String::from(k));
            println!("delete value: {}", v);
        })?;
        println!("key: {:?}", &key);
        println!("value: {}", value);

        let lookup = cache.lookup(&key.clone())?;
        match &lookup {
            None => {
                println!("value is none");
            }
            Some(v) => {
                println!("key: {}", String::from(v.key()));
                println!("value: {}", v.value());
            }
        }
        assert_eq!(value, *lookup.unwrap().value());

        cache.erase(&key)?;

        let lookup = cache.lookup(&key.clone())?;
        match &lookup {
            None => {
                println!("value is none");
            }
            Some(v) => {
                println!("key: {}", String::from(v.key()));
                println!("value: {}", v.value());
            }
        }
        assert_eq!(None, lookup);

        Ok(())
    }

    #[test]
    fn test_hash_map() {
        let mut map: HashMap<&str, &str> = HashMap::new();
        map.insert("123", "a");
        let value = map.get("123");
        match value {
            None => {}
            Some(v) => {
                println!("{}", v);
            }
        }
    }
}