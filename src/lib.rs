#![feature(core_ffi_c)]
#![feature(core_intrinsics)]
extern crate core;

pub mod db;
mod table;
pub mod util;
mod traits;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod test {

    #[test]
    pub fn test() {
        println!("hello world");
    }

}