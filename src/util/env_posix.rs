use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI32, Ordering};
use crate::util::status::{LevelError, Status};

static mut G_OPEN_READ_ONLY_FILE_LIMIT: i32 = -1;

//TODO
const K_DEFAULT_MMAP_LIMIT: u32 = 1000;

//TODO
static mut G_MMAP_LIMIT: u32 = K_DEFAULT_MMAP_LIMIT;

//TODO K_OPEN_BASE_FLAGS for what
const K_OPEN_BASE_FLAGS: u32 = 0;

//TODO use in __darwin_size_t
const K_WRITABLE_FILE_BUFFER_SIZE: u32 = 65536;

fn posix_error(context: &str, error_number: i32) -> Status {
    if error_number == 0 {
        Status::wrapper(LevelError::KNotFound, String::from(str).into())
    } else {
        Status::wrapper(LevelError::KIOError, String::from(str).into())
    }
}

// Helper class to limit resource usage to avoid exhaustion.
// Currently used to limit read-only file descriptors and mmap file usage
// so that we do not run out of file descriptors or virtual memory, or run into
// kernel performance problems for very large databases.
pub struct Limiter {
    // The number of available resources.
    //
    // This is a counter and is not tied to the invariants of any other class, so
    // it can be operated on safely using std::memory_order_relaxed.
    acquires_allowed: * AtomicI32,
}

impl Limiter {
    // If another resource is available, acquire it and return true.
    // Else return false.
    pub fn acquire(&self) -> bool {
        let old_acquires_allowed = self.acquires_allowed.fetch_add(1, Ordering::Relaxed);
        if old_acquires_allowed > 0 {
            return true;
        };
        self.acquires_allowed.fetch_add(1, Ordering::Relaxed);
        return false;
    }

    pub fn release(&self) {
        self.acquires_allowed.fetch_add(1, Ordering::Relaxed);
    }
}
