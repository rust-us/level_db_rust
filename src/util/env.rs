
use crate::util::Result;
use crate::util::slice::Slice;

pub struct  Env {}

pub struct WritableFile {}

impl Env {
    pub fn new_writable_file(&self, fname: &Slice) -> Result<WritableFile> {
        todo!()
    }
}