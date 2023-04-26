use crate::traits::DataIterator;
use crate::util::slice::Slice;
use crate::util::status::Status;
use crate::util::unsafe_slice::UnsafeSlice;

pub struct EmptyIterator {
    status: Box<Status>
}

impl DataIterator {
    pub fn valid(&self) -> bool {
        return false;
    }

    pub fn seek_to_first(&mut self) {

    }

    pub fn seek_to_last(&mut self) {

    }

    pub fn seek(&mut self, target: &Slice) {

    }

    pub fn next(&mut self) {

    }

    pub fn pre(&mut self) {

    }

    pub fn key(&self) -> UnsafeSlice {
        return UnsafeSlice::;
    }

    pub fn value(&self) -> UnsafeSlice {

    }

    fn status(&self) -> Status {
        return self.status();
    }
}