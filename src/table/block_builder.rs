use std::fs::File;
use std::sync::Arc;
use crate::util::options::Options;
use crate::util::slice::Slice;

use crate::util::Result;
use crate::util::status::Status;

// 智能指针 Rc<T>, 引用计数器，用来记录一个值是否被使用，如果计数为零可清除。
// 适用于堆中数据需要被程序多部分使用，但编译时不能确定谁最后完成。

// Arc 是一种能够使得数据在线程间安全共享的智能指针.
// Arc会追踪这个指针的所有拷贝，当最后一份拷贝离开作用域时，它就会安全释放内存。

// 智能指针 Box<T>。 box 允许你将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针。
pub struct BlockBuilder {
    // 在 BlockBuilder 初始化时，指定的配置项
    options: Box<Options>,
    index_block_options: Box<Options>,

    // SSTable 生成后的文件
    file: Arc<File>,

    offset: u64,
    status: Status,

    // 生成 SSTable 中的数据区域
    data_block: Arc<BlockBuilder>,
    // 生成 SSTable 中的数据索引区域
    index_block: Arc<BlockBuilder>,
}

impl BlockBuilder {
    pub fn new(options: &Options) -> Self {
        todo!()
    }

    /// 添加数据到block
    ///
    /// # Arguments
    ///
    /// * `key`: 键
    /// * `value`: 值
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn add(&mut self, _key: &Slice, _value: &Slice) {
        todo!()
    }

    /// 重置builder
    ///
    /// # Examples
    ///
    /// ```
    /// block_builder.reset();
    /// ```
    pub fn reset(&mut self) {
        todo!()
    }

    /// 构造block
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// let block = block_builder.finish();
    /// ```
    pub fn finish(&mut self) -> Result<Slice> {
        todo!()
    }

    /// 判断builder是否为空
    ///
    /// # Examples
    ///
    /// ```
    /// let is_empty = block_builder.empty();
    /// ```
    pub fn empty(&self) -> bool {
        todo!()
    }

    /// 估算当前的block大小
    ///
    /// # Examples
    ///
    /// ```
    /// let estimate_size = block_builder.current_size_estimate();
    /// ```
    pub fn current_size_estimate(&self) -> usize {
        todo!()
    }

}