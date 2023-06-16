use std::borrow::Borrow;
use std::fs::File;
use std::sync::Arc;
use crate::table::block_builder::BlockBuilder;
use crate::table::filter_block::{FilterBlock, FilterBlockBuilder, FilterBlockBuilderPtr};
use crate::table::format::BlockHandle;
use crate::traits::filter_policy_trait::FilterPolicy;
use crate::util::options::{CompressionType, OptionsPtr, Options};
use crate::util::slice::Slice;
use crate::util::status::Status;
use crate::util::unsafe_slice::UnsafeSlice;

/// 在一个 SSTable 中，文件末尾的 Footer 是定长的，
/// 其他数据都被划分成一个个变长的 block：
/// index block(@see format.BlockHandle、Footer#index_handle)、
/// meta_index block(@see format.BlockHandle、Footer#meta_index_handle)、
/// meta blocks(@see table.FilterBlock)、
/// data blocks。
pub struct TableBuilder {
    rep: Box<Rep>
}

/// TableBuilder Rep 结构体， 内部使用
struct Rep<> {
    options: OptionsPtr,
    index_block_options: OptionsPtr,

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
    filter_block: Option<FilterBlockBuilderPtr>,
    // 判断是否需要生成 SSTable中的数据索引， SSTable中每次生成一个完整的块之后，需要将该值置为 true， 说明需要为该块添加索引
    pending_index_entry: bool,
    // Handle to add to index block
    // pending_handle 记录需要生成数据索引的数据块在 SSTable 中的偏移量和大小
    // 也就是说， pending_handle 主要用于表示当前块的offset及size。
    pending_handle: BlockHandle,

    compressed_output: Slice,
}

impl TableBuilder {
    pub fn new_with_writable_file(options: OptionsPtr, writable_file: Arc<File>) -> Self {
        let mut rep = Rep::new(options, writable_file);

        // todo
        // if rep.filter_block.is_some() {
        //     rep.filter_block.unwrap().as_mut().start_block(0);
        // }

        Self {
            rep: Box::new(rep)
        }
    }

    /// 写入 entry
    pub fn add(&self, key: &UnsafeSlice, value: &UnsafeSlice) {
        todo!()
    }

    /// flush到文件
    pub fn flush(&self) {
        todo!()
    }

    /// block->Finish、压缩
    pub fn write_block(&self, block: &BlockBuilder, handler: &BlockHandle) {
        todo!()
    }

    /// datablock写入文件，添加压缩方式、crc。
    pub fn write_raw_block(&self, block_contents: &UnsafeSlice, compression_type: CompressionType, handler: &BlockHandle) {
        todo!()
    }

    pub fn status(&self) -> Status {
        todo!()
    }

    /// 剩余datablock写入文件，并生成管理区。
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
    pub fn new(opt: OptionsPtr, writableFile: Arc<File>) -> Self {
        let mut filter_block: Option<FilterBlockBuilderPtr>;
        if opt.filter_policy.is_none() {
            filter_block = None;
        }else {
            filter_block = Some(
                Arc::new(Box::new(
                    FilterBlockBuilder::new_with_policy(opt.filter_policy.clone().unwrap())
                ))
            );
        }
        // todo maybe try if let sytax
        let filter_block1 = opt.filter_policy.clone().map(
            |e| Arc::new(Box::new(FilterBlockBuilder::new_with_policy(e.clone())))
        );

        Self {
            options: opt.clone(),
            index_block_options: opt.clone(),
            file: writableFile,
            offset: 0,
            // default  Status::OK
            status: Status::default(),
            data_block: BlockBuilder::new(opt.clone()),
            index_block: BlockBuilder::new(opt.clone()),
            last_key: Slice::default(),
            num_entries: 0,
            closed: false,
            filter_block,
            pending_index_entry: false,
            pending_handle: BlockHandle::default(),
            compressed_output: Slice::default(),
        }
    }
}