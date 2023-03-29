use std::ffi::{c_char, c_void};
use std::ptr::{null, null_mut};
use std::sync::{Arc, Mutex};
use skiplist::OrderedSkipList;
use level_db_rust::db::skip_list::SkipList;
use level_db_rust::util::Arena;
use level_db_rust::util::arena::ArenaRef;
use level_db_rust::util::comparator::BytewiseComparatorImpl;
use level_db_rust::util::unsafe_slice::TryIntoUnsafeSlice;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

extern "C" fn write_cb(_: *mut c_void, message: *const c_char) {
    print!("{}", String::from_utf8_lossy(unsafe {
        std::ffi::CStr::from_ptr(message as *const i8).to_bytes()
    }));
}

fn mem_print() {
    unsafe { jemalloc_sys::malloc_stats_print(Some(write_cb), null_mut(), null()) }
}

fn bench_default_skiplist(mut list: SkipList<BytewiseComparatorImpl>, arena: ArenaRef, record_count: usize) {
    for j in 0..record_count {
        let value = format!("key_{}", j);
        list.insert(value.try_into_unsafe_slice(arena.clone()).unwrap()).unwrap();
    }
    println!("bench_default_skiplist: ");
    mem_print();
}

fn bench_skiplist_v_0_4_0(mut list: OrderedSkipList<String>, record_count: usize) {
    for j in 0..record_count {
        let value = format!("key_{}", j);
        list.insert(value.clone());
    }
    println!("bench_skiplist_v_0_4_0: ");
    mem_print();
}

fn main() {
    let record_count = 100 * 1024;
    // let cmp = Arc::new(BytewiseComparatorImpl::default());
    // let arena = Arc::new(Mutex::new(Arena::default()));
    // let list = SkipList::create(cmp, arena.clone());
    // bench_default_skiplist(list, arena, record_count);

    let list: OrderedSkipList<String> = unsafe {
        OrderedSkipList::with_comp(|a: &String, b: &String| {
            a.cmp(b)
        })
    };
    bench_skiplist_v_0_4_0(list, record_count);
}