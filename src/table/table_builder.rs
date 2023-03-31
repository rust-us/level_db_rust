use std::borrow::Borrow;
use std::fs::File;
use std::sync::Arc;
use crate::table::block_builder::BlockBuilder;
use crate::table::filter_block::FilterBlockBuilder;
use crate::table::format::BlockHandle;
use crate::traits::filter_policy_trait::FilterPolicy;
use crate::util::options::{CompressionType, Options};
use crate::util::slice::Slice;
use crate::util::status::Status;
use crate::util::unsafe_slice::UnsafeSlice;

pub struct TableBuilder {
    rep: Rep
}

struct Rep {
    // options: Box<Options>,
    // index_block_options: Options,
    file: Arc<File>,
    offset: u64,
    status: Status,
    // data_block: BlockBuilder,
    // index_block: BlockBuilder,
    last_key: Slice,
    num_entries: u64,
    // Either Finish() or Abandon() has been called.
    closed: bool,
}

impl TableBuilder {
    pub fn new_with_writable_file(options: &Options, writableFile: Arc<File>) -> Self {
        let rep = Rep::new(options, writableFile);

        // Self {
        //     rep
        // }

        todo!()
    }

    pub fn add(&self, key: &UnsafeSlice, value: &UnsafeSlice) {
        todo!()
    }

    pub fn flush(&self) {
        todo!()
    }

    pub fn write_block(&self, block: &BlockBuilder, handler: &BlockHandle) {
        todo!()
    }

    pub fn write_raw_block(&self, block_contents: &UnsafeSlice, compression_type: CompressionType, handler: &BlockHandle) {
        todo!()
    }

    pub fn status(&self) -> Status {
        todo!()
    }

    pub fn finish(&self) -> Status {
        todo!()
    }

    pub fn abandon(&self) {
        todo!()
    }

    pub fn get_num_entries(&self) -> u64 {
        todo!()
    }

    pub fn get_file_size(&self) -> u64 {
        todo!()
    }
}

impl Rep {
    pub fn new(opt: &Options, writableFile: Arc<File>) -> Self {
        Self {
            // options: Box::new(*opt),
            file: writableFile,
            offset: 0,
            // todo default  Status::OK
            status: Status::default(),
            // data_block: BlockBuilder::new(&opt),
            last_key: Default::default(),
            num_entries: 0,
            closed: false,
        }
    }
}