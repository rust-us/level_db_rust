use crate::util::mutex_lock::MutexLock;
use crate::util::status::Status;


pub trait Slice {}

//TODO format c++ code comment with rust markdown
pub trait Env {
    // Return a default environment suitable for the current operating
    // system.  Sophisticated users may wish to provide their own Env
    // implementation instead of relying on this default environment.
    //
    // The result of Default() belongs to leveldb and must never be deleted.
    fn default() -> Self;

    // Create an object that sequentially reads the file with the specified name.
    // On success, stores a pointer to the new file in *result and returns OK.
    // On failure stores nullptr in *result and returns non-OK.  If the file does
    // not exist, returns a non-OK status.  Implementations should return a
    // NotFound status when the file does not exist.
    //
    // The returned file will only be accessed by one thread at a time.
    fn new_sequential_file(fname: &str, result: dyn SequentialFile) -> Status;

    // Create an object supporting random-access reads from the file with the
    // specified name.  On success, stores a pointer to the new file in
    // *result and returns OK.  On failure stores nullptr in *result and
    // returns non-OK.  If the file does not exist, returns a non-OK
    // status.  Implementations should return a NotFound status when the file does
    // not exist.
    //
    // The returned file may be concurrently accessed by multiple threads.
    fn new_random_access_file(fname: &str, result: dyn SequentialFile) -> Status;

    // Create an object that writes to a new file with the specified
    // name.  Deletes any existing file with the same name and creates a
    // new file.  On success, stores a pointer to the new file in
    // *result and returns OK.  On failure stores nullptr in *result and
    // returns non-OK.
    //
    // The returned file will only be accessed by one thread at a time.
    fn new_writable_file(fname: &str, result: dyn SequentialFile) -> Status;

    // Create an object that either appends to an existing file, or
    // writes to a new file (if the file does not exist to begin with).
    // On success, stores a pointer to the new file in *result and
    // returns OK.  On failure stores nullptr in *result and returns
    // non-OK.
    //
    // The returned file will only be accessed by one thread at a time.
    //
    // May return an IsNotSupportedError error if this Env does
    // not allow appending to an existing file.  Users of Env (including
    // the leveldb implementation) must be prepared to deal with
    // an Env that does not support appending.
    fn new_appendable_file(fname: &str, result: dyn WritableFile) -> Status;

    // Returns true iff the named file exists.
    fn file_exists(fname: &str) -> bool;

    // Store in *result the names of the children of the specified directory.
    // The names are relative to "dir".
    // Original contents of *results are dropped.
    fn get_children(dir: &str, result: Vec<String>) -> bool;

    // Delete the named file.
    //
    // The default implementation calls DeleteFile, to support legacy Env
    // implementations. Updated Env implementations must override RemoveFile and
    // ignore the existence of DeleteFile. Updated code calling into the Env API
    // must call RemoveFile instead of DeleteFile.
    //
    // A future release will remove DeleteDir and the default implementation of
    // RemoveDir.
    fn remove_file(fname: &str) -> Status;

    // DEPRECATED: Modern Env implementations should override RemoveFile instead.
    //
    // The default implementation calls RemoveFile, to support legacy Env user
    // code that calls this method on modern Env implementations. Modern Env user
    // code should call RemoveFile.
    //
    // A future release will remove this method.
    fn delete_file(fname: &str) -> Status;

    // Create the specified directory.
    fn create_dir(fname: &str) -> Status;

    // Delete the specified directory.
    //
    // The default implementation calls DeleteDir, to support legacy Env
    // implementations. Updated Env implementations must override RemoveDir and
    // ignore the existence of DeleteDir. Modern code calling into the Env API
    // must call RemoveDir instead of DeleteDir.
    //
    // A future release will remove DeleteDir and the default implementation of
    // RemoveDir.
    fn remove_dir(dirname: &str) -> Status;

    // DEPRECATED: Modern Env implementations should override RemoveDir instead.
    //
    // The default implementation calls RemoveDir, to support legacy Env user
    // code that calls this method on modern Env implementations. Modern Env user
    // code should call RemoveDir.
    //
    // A future release will remove this method.
    fn delete_dir(dirname: &str) -> Status;

    // Store the size of fname in *file_size.
    fn get_file_size(fname: &str, file_size: u64) -> Status;

    // Rename file src to target.
    fn rename_file(src: &str, target: &str) -> Status;


    // Lock the specified file.  Used to prevent concurrent access to
    // the same db by multiple processes.  On failure, stores nullptr in
    // *lock and returns non-OK.
    //
    // On success, stores a pointer to the object that represents the
    // acquired lock in *lock and returns OK.  The caller should call
    // UnlockFile(*lock) to release the lock.  If the process exits,
    // the lock will be automatically released.
    //
    // If somebody else already holds the lock, finishes immediately
    // with a failure.  I.e., this call does not wait for existing locks
    // to go away.
    //
    // May create the named file if it does not already exist.
    fn lock_file(fname: &str, lock: &dyn FileLock) -> Status;

    // Release the lock acquired by a previous successful call to LockFile.
    // REQUIRES: lock was returned by a successful LockFile() call
    // REQUIRES: lock has not already been unlocked.
    fn unlock_file(fname: &str, lock: &dyn FileLock) -> Status;

    // Arrange to run "(*function)(arg)" once in a background thread.
    //
    // "function" may run in an unspecified thread.  Multiple functions
    // added to the same Env may run concurrently in different threads.
    // I.e., the caller may not assume that background work items are
    // serialized.
    //TODO need discuss about how to impl it
    fn schedule(fn1: fn(), lock: &dyn FileLock) -> Status;

    // Start a new thread, invoking "function(arg)" within the new thread.
    // When "function(arg)" returns, the thread will be destroyed.
    //TODO need discuss about how to impl it
    fn start_thread();

    // *path is set to a temporary directory that can be used for testing. It may
    // or may not have just been created. The directory may or may not differ
    // between runs of the same process, but subsequent calls will return the
    // same directory.
    fn get_test_directory(path: &str) -> Status;

    // Create and return a log file for storing informational messages.
    fn new_logger(fname: &str, result: &dyn Logger) -> Status;

    // Returns the number of micro-seconds since some fixed point in time. Only
    // useful for computing deltas of time.
    fn now_micros() -> u64;

    // Sleep/delay the thread for the prescribed number of micro-seconds.
    fn sleep_for_microseconds(micros: i32);
}

pub trait SequentialFile {
    // Read up to "n" bytes from the file.  "scratch[0..n-1]" may be
    // written by this routine.  Sets "*result" to the data that was
    // read (including if fewer than "n" bytes were successfully read).
    // May set "*result" to point at data in "scratch[0..n-1]", so
    // "scratch[0..n-1]" must be live when "*result" is used.
    // If an error was encountered, returns a non-OK status.
    //
    // REQUIRES: External synchronization
    fn read(self ,n: usize, result: &Vec<u8>, scratch: &String) -> Stats;

    fn skip(self, n: u64) -> Status;
}

pub trait RandomAccessFile {
    fn read(self,offset: u64, n: usize, result: &Vec<char>, scratch: &String) -> Status;
}

pub trait WritableFile {
    fn append(data: &Vec<char>) -> Status;
    fn close() -> Status;
    fn flush() -> Status;
    fn sync() -> Status;
}

pub trait Logger {

    // fn Logv(format: &str,)
}

pub trait FileLock {
    fn file_lock();
}

//TODO use info_log to log the message, it depend on logger
macro_rules! log {
    ($info_log:expr,$format:expr, $(eval $es:expr),+) => {{

    }}
}


//TODO  it depend on Env implementation
fn do_write_string_to_file(env: &dyn Env, data: &Vec<&str>, fname: &str, should_sync: bool) -> Status {
    // WritableFile* file;
    // let file: &WritableFile;
    // env.
    Status
}


fn write_string_to_file(env: &dyn Env, data: &Vec<&str>, fname: &str) -> Status {
    do_write_string_to_file(env, data, fname, false)
}

//TODO  it depend on Env implementation
fn read_file_to_string(env: &dyn Env, fname: &str, data: &str) -> Status {
    Status
}

fn write_string_to_file_sync(env: &dyn Env, data: &Vec<&str>, fname: &str) -> Status {
    do_write_string_to_file(env, data, fname, true)
}

pub struct EnvWrapper{
    target : *Env
}

// impl Env for EnvWrapper {
//     fn default() -> Self {
//
//     }
//
//     fn new_sequential_file(fname: &str, result: dyn SequentialFile) -> Status {
//         todo!()
//     }
//
//     fn new_random_access_file(fname: &str, result: dyn SequentialFile) -> Status {
//         todo!()
//     }
//
//     fn new_writable_file(fname: &str, result: dyn SequentialFile) -> Status {
//         todo!()
//     }
//
//     fn new_appendable_file(fname: &str, result: dyn WritableFile) -> Status {
//         todo!()
//     }
//
//     fn file_exists(fname: &str) -> bool {
//         todo!()
//     }
//
//     fn get_children(dir: &str, result: Vec<String>) -> bool {
//         todo!()
//     }
//
//     fn remove_file(fname: &str) -> Status {
//         todo!()
//     }
//
//     fn delete_file(fname: &str) -> Status {
//         todo!()
//     }
//
//     fn create_dir(fname: &str) -> Status {
//         todo!()
//     }
//
//     fn remove_dir(dirname: &str) -> Status {
//         todo!()
//     }
//
//     fn delete_dir(dirname: &str) -> Status {
//         todo!()
//     }
//
//     fn get_file_size(fname: &str, file_size: u64) -> Status {
//         todo!()
//     }
//
//     fn rename_file(src: &str, target: &str) -> Status {
//         todo!()
//     }
//
//     fn lock_file(fname: &str, lock: &dyn FileLock) -> Status {
//         todo!()
//     }
//
//     fn unlock_file(fname: &str, lock: &dyn FileLock) -> Status {
//         todo!()
//     }
//
//     fn schedule(fn1: fn(), lock: &dyn FileLock) -> Status {
//         todo!()
//     }
//
//     fn start_thread() {
//         todo!()
//     }
//
//     fn get_test_directory(path: &str) -> Status {
//         todo!()
//     }
//
//     fn new_logger(fname: &str, result: &dyn Logger) -> Status {
//         todo!()
//     }
//
//     fn now_micros() -> u64 {
//         todo!()
//     }
//
//     fn sleep_for_microseconds(micros: i32) {
//         todo!()
//     }
// }







