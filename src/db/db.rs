use std::ops::Range;
use crate::traits::DataIterator;
use crate::util::options::{Options, ReadOptions, WriteOptions};
use crate::util::Result;
use crate::util::slice::Slice;

struct DB {

}

//TODO temp
struct WriteBatchOptions {}
struct WriteBatch {}
pub trait Snapshot {}

impl DB {

    async fn open(_opt: Options, _name: String) -> Result<Self> {
        todo!()
    }

    async fn put(&mut self, _opt: WriteOptions, _key: Slice, _value: Slice) -> Result<()> {
        todo!()
    }

    async fn delete(&mut self, _key: Slice) -> Result<bool> {
        todo!()
    }

    async fn write(&mut self, _opt: WriteBatchOptions, _updates: WriteBatch) -> Result<()> {
        todo!()
    }

    async fn get(&mut self, _opt: ReadOptions, _key: Slice) -> Result<Option<Slice>> {
        todo!()
    }

    fn new_iterator(&mut self, _opt: ReadOptions) -> Result<Box<dyn DataIterator>> {
        todo!()
    }

    fn get_snapshot(&self) -> Box<dyn Snapshot> {
        todo!()
    }

    fn release_snapshot(&mut self, _snap: Box<dyn Snapshot>) -> Result<()> {
        todo!()
    }

    fn get_property(&self, _key: Slice) -> Option<Slice> {
        todo!()
    }

    fn get_approximate_sizes(&self, _rng: Range<Slice>, _n: u32) -> Result<Vec<usize>> {
        todo!()
    }

    async fn compact_range(&mut self, _rng: Range<Slice>) -> Result<()> {
        todo!()
    }

}