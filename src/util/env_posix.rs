use std::error;
use std::fs::File;
use std::io::{ErrorKind, Read, Seek, SeekFrom};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread::sleep;
use crate::util::env::{RandomAccessFile, SequentialFile};
use crate::util::slice::Slice;
use crate::util::status::{LevelError, Status};
use crate::util::status::LevelError::KInvalidArgument;

static mut G_OPEN_READ_ONLY_FILE_LIMIT: i32 = -1;

//TODO
const K_DEFAULT_MMAP_LIMIT: u32 = 1000;

//TODO
static mut G_MMAP_LIMIT: u32 = K_DEFAULT_MMAP_LIMIT;

//TODO K_OPEN_BASE_FLAGS for what
const K_OPEN_BASE_FLAGS: u32 = 0;

//TODO use in __darwin_size_t
const K_WRITABLE_FILE_BUFFER_SIZE: u32 = 65536;

fn posix_error(context: &str, error_number: u64) -> Status {
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

pub struct PosixSequentialFile<'a> {
    fd: &'a mut File,
    filename: &'a str,
}

impl SequentialFile for PosixSequentialFile {
    fn read(&self, n: &mut usize, result: &mut Vec<u8>, scratch: &String) -> Status {
        while true {
            let res = self.fd.read(result);
            *n = match res {
                Ok(n) => n,
                // Read error.
                Err(err) => match error.kind() {
                    // Retry
                    ErrorKind::Interrupted => continue,
                    other_error => {
                        // Read error.
                        posix_error(self.filename, other_error)
                    }
                }
            };
            *result = Slice(scratch, read_size);
            break;
        }
        return Status::default();
    }

    fn skip(&self, n: u64) -> Status {
        let result = self.fd.seek(SeekFrom::Start(n));
        let f = match result {
            Ok(n) => Status::default(),
            Err(error) => posix_error(self.filename, other_error),
        };
    }
}


struct PosixRandomAccessFile<'a> {
    has_permanent_fd: bool,
    fd: &'a mut File,
    fd_limiter: &'a Limiter,
    filename: &'a str,
}

impl RandomAccessFile for PosixRandomAccessFile {
    fn read(mut self, offset: u64, n: usize, result: &mut Vec<u8>, scratch: &String) -> Status {
        if !self.has_permanent_fd {
            let res = &mut File::open(self.filename);
            match res {
                Ok(file) => self.fd = file,
                Err(err) => return posix_error(self.filename, err.into()),
            };
        }
        let res = self.fd.read(result);
        *n = match res {
            Ok(n) => n,
            // Read error.
            Err(err) => return posix_error(self.filename, err.into())
        };
        *result = Slice(scratch, read_size);

        Status::default()
    }
}


// Implements random read access in a file using mmap().
//
// Instances of this class are thread-safe, as required by the RandomAccessFile
// API. Instances are immutable and Read() only calls thread-safe library
// functions.
//TODO
struct PosixMmapReadableFile<'a> {
    mmap_base: &'a mut Vec<char>,
    length: usize,
    mmap_limiter: &'a Limiter,
    filename: &'a str,

}

impl RandomAccessFile for PosixMmapReadableFile {
    fn read(self, offset: u64, n: usize, result: &mut Vec<char>, scratch: &String) -> Status {
        if offset + n > self.length as u64 {
            return posix_error(self.filename, Status::KInvalidArgument);
        };
        *result = self.mmap_base[offset: n];
        Status::default()
    }
}

struct PosixWritableFile<'a> {
    buf: &'a mut Vec<char>,
    pos: usize,
    fd: &'a mut File,
    is_manifest: bool,
    filename: &'a str,
    dirname: &'a str,
}















