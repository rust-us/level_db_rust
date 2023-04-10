use criterion::{Criterion, criterion_group, criterion_main};
use rand::RngCore;
use level_db_rust::util::crc::CRC;

pub const SRC_DATA: [u8; 512] = [0; 512];

pub fn default_crc_bench(c: &mut Criterion) {
    let mut rnd = rand::thread_rng();
    c.bench_function("default_crc", |b| {
        b.iter(|| {
            rnd.fill_bytes(&mut SRC_DATA);
            CRC::value(&SRC_DATA);
        });
    });
    c.bench_function("crc32fast", |b| {
        b.iter(|| {
            rnd.fill_bytes(&mut SRC_DATA);
            crc32fast::hash(&SRC_DATA);
        });
    });
}

criterion_group!(benches, default_crc_bench);
criterion_main!(benches);