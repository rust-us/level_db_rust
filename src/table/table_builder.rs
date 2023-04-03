use std::borrow::Borrow;
use std::fs::File;
use std::sync::Arc;
use crate::table::block_builder::BlockBuilder;
use crate::table::filter_block::{FilterBlock, FilterBlockBuilder};
use crate::table::format::BlockHandle;
use crate::traits::filter_policy_trait::FilterPolicy;
use crate::util::options::{CompressionType, OptionRef, Options};
use crate::util::slice::Slice;
use crate::util::status::Status;
use crate::util::unsafe_slice::UnsafeSlice;

pub struct TableBuilder {
    rep: Box<Rep>
}

/// TableBuilder Rep 结构体， 内部使用
struct Rep<> {
    options: Box<Options>,
    index_block_options: Box<Options>,

    // SSTable 生成后的文件
    file: Arc<File>,

    offset: u64,
    status: Status,

    // 生成 SSTable 中的数据区域
    data_block: BlockBuilder,
    // 生成 SSTable 中的数据索引区域
    index_block: BlockBuilder,

    last_key: Slice,
    num_entries: u64,
    // Either Finish() or Abandon() has been called.
    closed: bool,

    // 生成 SSTable 中的元数据区域
    filter_block: Option<FilterBlockBuilder>,
    // 判断是否需要生成 SSTable中的数据索引， SSTable中每次生成一个完整的块之后，需要将该值置为 true， 说明需要为该块添加索引
    pending_index_entry: bool,
    // Handle to add to index block
    // pending_handle 记录需要生成数据索引的数据块在 SSTable 中的偏移量和大小
    pending_handle: BlockHandle,

    compressed_output: Slice,
}

impl TableBuilder {
    pub fn new_with_writable_file(options: &Options, writable_file: Arc<File>) -> Self {
        let rep = Rep::new(options, writable_file);

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
    pub fn new(opt: OptionRef, writableFile: Arc<File>) -> Self {
        let options = Box::new(opt.clone());
        let index_block_options = Box::new(opt.clone());

        let mut filter_block: Option<FilterBlockBuilder>;
        if opt.filter_policy.is_none() {
            filter_block = None;
        }else {
            filter_block = Some(FilterBlockBuilder::new_with_policy(opt.filter_policy.unwrap().clone()));
        }

        Self {
            options,
            index_block_options,
            file: writableFile,
            offset: 0,
            // default  Status::OK
            status: Status::default(),
            data_block: BlockBuilder::new(&options.as_ref()),
            index_block: BlockBuilder::new(&index_block_options.as_ref()),
            last_key: Default::default(),
            num_entries: 0,
            closed: false,
            filter_block,
            pending_index_entry: false,
            pending_handle: Default::default(),
            compressed_output: Default::default(),
        }
    }
}