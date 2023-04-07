use std::fs::File;
use crate::util::Result;
use crate::util::slice::Slice;

pub struct  Env {}

impl Env {
    pub fn new_writable_file(&self, fname: &Slice) -> Result<File> {
        todo!()
    }
}