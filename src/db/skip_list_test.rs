// mod test {
//     use std::collections::HashSet;
//     use std::env::args;
//     use std::ffi::{c_char, c_void};
//     use std::ptr::{null, null_mut};
//     use std::sync::{Arc, Mutex};
//
//     use rand::Rng;
//     use skiplist::OrderedSkipList;
//
//     use crate::db::DefaultSkipList;
//     use crate::db::skip_list::SkipList;
//     use crate::debug;
//     use crate::util::Arena;
//     use crate::util::arena::ArenaRef;
//     use crate::util::comparator::BytewiseComparatorImpl;
//     use crate::util::mem_debug::mem_print;
//     use crate::util::Result;
//     use crate::util::slice::Slice;
//     use crate::util::unsafe_slice::TryIntoUnsafeSlice;
//
//     #[test]
//     fn test_add() -> Result<()> {
//         let cmp = Arc::new(BytewiseComparatorImpl::default());
//         let arena = Arc::new(Mutex::new(Arena::default()));
//         let mut list = DefaultSkipList::create(cmp, arena.clone());
//         let len = 10;
//         for i in 0..len {
//             list.insert(format!("key_{}", i).try_into_unsafe_slice(arena.clone())?).expect("insert ok");
//         }
//         assert_eq!(10, list.len(), "expect 10, but actually is: {}", list.len());
//         debug!("{}", list.to_string());
//         for i in 0..len {
//             let key: Slice = format!("key_{}", i).into();
//             debug!("contains key: {}", key);
//             assert!(list.contains(&key), "contains key: {}", key);
//         }
//         list.iter().for_each(|slice| {
//             debug!("slice: {}", slice.as_str())
//         });
//         Ok(())
//     }
//
//     #[test]
//     fn test_rnd_add() -> Result<()> {
//         let cmp = Arc::new(BytewiseComparatorImpl::default());
//         let arena = Arc::new(Mutex::new(Arena::default()));
//         let mut list = DefaultSkipList::create(cmp, arena.clone());
//         let len = 10;
//         let mut rnd = rand::thread_rng();
//         let mut set = HashSet::new();
//         for _i in 0..10 {
//             let j = rnd.gen_range(0..len);
//             let key = format!("key_{}", j);
//             set.insert(key.clone());
//             list.insert(key.try_into_unsafe_slice(arena.clone())?)?;
//             debug!("skiplist: {}", list.to_string());
//         }
//         assert_eq!(set.len(), list.len(), "list length must eq: {}", list.len());
//         set.iter().for_each(|key| {
//             let c = list.contains(&key);
//             assert!(c, "must contains key: {}", key)
//         });
//
//         Ok(())
//     }
//
//
//     fn default_skiplist(mut list: SkipList<BytewiseComparatorImpl>, arena: ArenaRef, record_count: usize) {
//         for j in 0..record_count {
//             let value = format!("key_{}", j);
//             list.insert(value.try_into_unsafe_slice(arena.clone()).unwrap()).unwrap();
//         }
//         println!("bench_default_skiplist: ");
//         mem_print();
//     }
//
//     fn bench_skiplist_v_0_4_0(mut list: OrderedSkipList<String>, record_count: usize) {
//         for j in 0..record_count {
//             let value = format!("key_{}", j);
//             list.insert(value.clone());
//         }
//         println!("bench_skiplist_v_0_4_0: ");
//         mem_print();
//     }
//
//     #[test]
//     fn bench_default_skiplist() {
//         let record_count = 100 * 1024;
//         println!("bench default skiplist");
//         let cmp = Arc::new(BytewiseComparatorImpl::default());
//         let arena = Arc::new(Mutex::new(Arena::default()));
//         let list = SkipList::create(cmp, arena.clone());
//         default_skiplist(list, arena, record_count);
//     }
//
//     #[test]
//     fn bench_crate_skiplist() {
//         let record_count = 100 * 1024;
//         println!("bench crate skiplist");
//         let list: OrderedSkipList<String> = unsafe {
//             OrderedSkipList::with_comp(|a: &String, b: &String| {
//                 a.cmp(b)
//             })
//         };
//         bench_skiplist_v_0_4_0(list, record_count);
//     }
// }