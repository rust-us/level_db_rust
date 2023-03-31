use std::borrow::Borrow;
use std::sync::Arc;
use crate::table::block_builder::BlockBuilder;
use crate::table::filter_block::FilterBlockBuilder;
use crate::traits::filter_policy_trait::FilterPolicy;
use crate::util::env::WritableFile;
use crate::util::options::Options;
use crate::util::slice::Slice;
use crate::util::status::Status;

pub struct TableBuilder {
    rep: Rep
}

struct Rep {
    // options: Box<Options>,
    // index_block_options: Options,
    file: Arc<WritableFile>,
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
    pub fn new_with_writable_file(options: &Options, writableFile: Arc<WritableFile>) -> Self {
        let rep = Rep::new(options, writableFile);

        // Self {
        //     rep
        // }

        todo!()
    }

    pub fn add(&self, key: &Slice, value: &Slice) {
        todo!()
    }
}

impl Rep {
    pub fn new(opt: &Options, writableFile: Arc<WritableFile>) -> Self {
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