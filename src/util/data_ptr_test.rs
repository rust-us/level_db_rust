use std::alloc::{alloc, Layout};
use crate::util::data_ptr::DataPtr;

#[test]
fn test_create() {
    let raw_ptr = unsafe {
        alloc(Layout::from_size_align(8, 8).unwrap())
    };
    let ptr = DataPtr::from(raw_ptr);
    unsafe {
        ptr.write(18_u32);
        ptr.write(32_u32);
    }
}