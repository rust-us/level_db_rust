use std::cmp::{min, Ordering};
use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::sync::Arc;
use crate::util::coding::Encoder;
use crate::util::options::{Options, OptionsPtr};
use crate::util::slice::Slice;

use crate::util::Result;
use crate::util::status::Status;

// 智能指针 Rc<T>, 引用计数器，用来记录一个值是否被使用，如果计数为零可清除。
// 适用于堆中数据需要被程序多部分使用，但编译时不能确定谁最后完成。

// Arc 是一种能够使得数据在线程间安全共享的智能指针.
// Arc会追踪这个指针的所有拷贝，当最后一份拷贝离开作用域时，它就会安全释放内存。

// 智能指针 Box<T>。 box 允许你将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针。

/// BlockBuilder 的 `Arc<BlockBuilder>` 别名
pub type BlockBuilderPtr = Arc<BlockBuilder>;

/// 生成块
pub struct BlockBuilder {
    // 在 BlockBuilder 初始化时，指定的配置项
    options: OptionsPtr,

    // 目标缓冲区，也就是按照输出格式处理好的内存区域
    buffer: Vec<u8>,

    // Restart points
    restarts: Vec<usize>,

    // Number of entries emitted since restart
    counter: u32,
    // Has Finish() been called?
    finished: bool,

    last_key: String
}

impl BlockBuilder {
    pub fn new(options: OptionsPtr) -> Self {
        assert!(options.block_restart_interval >= 1);

        let mut restarts = vec![];
        // First restart point is at offset 0
        restarts.push(0);

        Self {
            options,
            buffer: vec![],
            restarts,
            counter: 0,
            finished: false,
            last_key: "".to_string(),
        }
    }

    pub fn get_restarts(self) -> Vec<usize> {
        self.restarts
    }

    /// 向datablock增加entry
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
    pub fn add(&mut self, key: Slice, value: Slice) {
        let last_key_piece = Slice::from(&self.last_key);
        assert!(!self.finished);
        assert!(self.counter <= self.options.block_restart_interval);
        assert!(!self.buffer.is_empty() // No values yet?
            //  > 0
            || self.options.cmp.compare(key.deref(), last_key_piece.deref()).unwrap() != Ordering::Less
        );

        let mut shared = 0;

        if self.counter < self.options.block_restart_interval {
            // See how much sharing to do with previous string
            let min_length = min(last_key_piece.size(), key.len());
            while ((shared < min_length) && (last_key_piece[shared] == key[shared])) {
                shared += 1;
            }
        }else {
            // Restart compression
            self.restarts.push(self.buffer.len());
            self.counter = 0;
        }

        let non_shared = key.size() - shared;

        // Add "<shared><non_shared><value_size>" to buffer_
        let mut encoder = Encoder::with_buf(&mut self.buffer);
        encoder.put_varint32(shared as u32);
        encoder.put_varint32(non_shared as u32);
        encoder.put_varint32(value.size() as u32);

        // Add string delta to buffer_ followed by value
        // is buffer_.append(key.data() + shared, non_shared);
        self.buffer.write(key.deref());

        // is  buffer_.append(value.data(), value.size());
        self.buffer.write(value.deref());

        // Update state
        // last_key_.resize(shared);
        self.last_key.reserve(shared);
        self.last_key.push(key.as_str());

        // last_key_.append(key.data() + shared, non_shared);
        // assert(Slice(last_key_) == key);
        // counter_++;
    }

    /// 重置builder
    ///
    /// # Examples
    ///
    /// ```
    /// block_builder.reset();
    /// ```
    pub fn reset(&mut self) {
        self.buffer.clear();

        self.restarts.clear();
        // First restart point is at offset 0
        self.restarts.push(0);

        self.counter = 0;
        self.finished = false;
        self.last_key.clear();
    }

    /// 追加Restart points
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

    /// 估算当前的block大小, 超过一定大小后，写入文件
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