use std::ffi::{c_char, c_void};
use std::ptr::{null, null_mut};

extern "C" fn write_cb(_: *mut c_void, message: *const c_char) {
    print!("{}", String::from_utf8_lossy(unsafe {
        std::ffi::CStr::from_ptr(message as *const i8).to_bytes()
    }));
}

pub fn mem_print() {
    unsafe { jemalloc_sys::malloc_stats_print(Some(write_cb), null_mut(), null()) }
}