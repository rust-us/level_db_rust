use std::fs::File;
use std::sync::Arc;
use crate::util::options::Options;
use crate::util::Result;
use crate::util::status::Status;

pub struct Table {
    rep: Rep
}

struct Rep {
    options: Box<Options>,
    status: Status,
    file: Arc<File>,
}

impl Table {
    pub fn new() -> Self{
        todo!()
    }

    pub fn open(&self, options:&Options, randomAccessFile:&File, file_size: u64, table:&Table) -> Result<bool>{
        todo!()
    }
}