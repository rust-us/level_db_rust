use std::sync::{Arc, Mutex};

use criterion::{Criterion, criterion_group, criterion_main};
use rand::Rng;
use skiplist::OrderedSkipList;

use level_db_rust::db::skip_list::SkipList;
use level_db_rust::util::Arena;
use level_db_rust::util::arena::ArenaRef;
use level_db_rust::util::comparator::BytewiseComparatorImpl;
use level_db_rust::util::unsafe_slice::TryIntoUnsafeSlice;


const BENCH_TIMES: usize = 128;

pub fn skiplist_bench(c: &mut Criterion) {
    // 生成测试样本,保证两次测试都是相同的次数
    let mut rnd = rand::thread_rng();
    let mut every_bench_times = [0; BENCH_TIMES];
    for i in 0..BENCH_TIMES {
        every_bench_times[i] = rnd.gen_range(32..20480);
    }

    c.bench_function("default_skiplist", |b| {
        let mut i = 0;
        b.iter(|| {
            let cmp = Arc::new(BytewiseComparatorImpl::default());
            let arena = Arc::new(Mutex::new(Arena::default()));
            let list = SkipList::create(cmp, arena.clone());
            bench_default_skiplist(list, arena, every_bench_times[i % BENCH_TIMES]);
            i += 1;
        });
    });

    c.bench_function("skiplist-0.4.0", |b| {
        let mut i = 0;
        b.iter(|| {
            let list: OrderedSkipList<String> = unsafe {
                OrderedSkipList::with_comp(|a: &String, b: &String| {
                    a.cmp(b)
                })
            };
            bench_skiplist_v_0_4_0(list, every_bench_times[i % BENCH_TIMES]);
            i += 1;
        });
    });
}

fn bench_default_skiplist(mut list: SkipList<BytewiseComparatorImpl>, arena: ArenaRef, record_count: usize) {
    for j in 0..record_count {
        let value = format!("key_{}", j);
        list.insert(value.try_into_unsafe_slice(arena.clone()).unwrap()).unwrap();
    }
}

fn bench_skiplist_v_0_4_0(mut list: OrderedSkipList<String>, record_count: usize) {
    for j in 0..record_count {
        let value = format!("key_{}", j);
        list.insert(value.clone());
    }
}

criterion_group!(benches, skiplist_bench);
criterion_main!(benches);