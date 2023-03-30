use std::{mem, slice};
use std::alloc::{alloc, Layout};
use std::io::Write;

use criterion::{Criterion, criterion_group, criterion_main};
use level_db_rust::debug;

pub fn u32_shift_bench(c: &mut Criterion) {
    let mut data = [0_u8; 4];
    let mut buf = data.as_mut_slice();
    let value = 12345678_u32;
    let mut g = c.benchmark_group("u32_shift");

    g.bench_function("to_ne_bytes", |g| {
        g.iter(|| {
            buf.write(&value.to_be_bytes()).unwrap();
        });
    });
    buf = data.as_mut_slice();
    buf.fill(0); // reset
    debug!("is big endian: {}", cfg!(target_endian = "big"));
    g.bench_function("raw_write", |g| {
        g.iter(|| {
            unsafe {
                if cfg!(target_endian = "big") {
                    (buf.as_mut_ptr() as *mut u32).write(value);
                } else {
                    (buf.as_mut_ptr() as *mut u32).write(value.swap_bytes());
                }
            }
        });
    });
    buf = data.as_mut_slice();
    buf.fill(0); // reset
    g.bench_function("shift_bytes", |g| {
        g.iter(|| {
            buf[0] = ((value >> 0) & 0xff) as u8;
            buf[1] = ((value >> 1) & 0xff) as u8;
            buf[2] = ((value >> 2) & 0xff) as u8;
            buf[3] = ((value >> 3) & 0xff) as u8;
        });
    });
}

criterion_group!(benches, u32_shift_bench);
criterion_main!(benches);