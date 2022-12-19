use crate::util::status::Status;

pub trait Env {
    fn new_appendable_file(fname: &str, result: &dyn WritableFile) -> Status;
    fn remove_dir(dirname: &str) -> Status;
    fn delete_dir(dirname: &str) -> Status;
    fn remove_file(fname: &str) -> Status;
    fn delete_file(fname: &str) -> Status;
}

pub trait SequentialFile {}

pub trait RandomAccessFile {}

pub trait WritableFile {}

pub trait Logger {}

pub trait FileLock {}

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

fn write_string_to_file_sync(env: &dyn Env, data: &Vec<&str>, fname: &str) -> Status {
    do_write_string_to_file(env, data, fname, true)
}


//TODO  it depend on Env implementation
fn read_file_to_string(env: &dyn Env, fname: &str, data: &str) -> Status {
    Status
}








