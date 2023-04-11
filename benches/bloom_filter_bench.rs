use std::borrow::Borrow;
use criterion::{Criterion, criterion_group, criterion_main};
use rand::Rng;
use level_db_rust::util::coding::Coding;
use level_db_rust::util::filter_policy_bloom::BloomFilterPolicy;
use level_db_rust::util::slice::Slice;

const KEY_SIZE: usize = 10_000_000;
const BENCH_TIMES: usize = 128;

/// BloomFilter bench Test
pub fn bloom_filter_bench(c: &mut Criterion) {
    let data: Vec<&Slice> = vec![&Slice::default(); KEY_SIZE];
    for i in 0..KEY_SIZE {
        data[i] = format!("{}", i).into();
    }

    let mut every_bench_times = [0; BENCH_TIMES];
    for i in 0..BENCH_TIMES {
        every_bench_times[i] = rnd.gen_range(32..20480);
    }

    c.bench_function("default_test", |b| {
        let mut i = 0;
        b.iter(|| {
            let filter = BloomFilterPolicy::new();
            let bloom_filter_data = filter.create_filter_with_len(KEY_SIZE, data);

            bench_default(filter, &bloom_filter_data, every_bench_times[i % BENCH_TIMES]);
            i += 1;
        });
    });
}

fn bench_default(filter: BloomFilterPolicy, bloom_filter_data: &Slice, record_count: usize) {
    for j in 0..record_count {
        let key_may_match = filter.key_may_match(format!("{}", i).into(), bloom_filter_data);
        assert!(key_may_match)
    }

    for j in (KEY_SIZE+1)..(KEY_SIZE+100) {
        let key_may_match = filter.key_may_match(format!("{}", i).into(), bloom_filter_data);
        // key_may_match 可能为 true， 可能为 false
        println!("key_may_match:{}.", key_may_match)
    }
}

criterion_group!(benches, skiplist_bench);
criterion_main!(benches);