use crate::traits::DataIterator;
use crate::util::slice::Slice;
use crate::util::Result;

struct Saver {}

struct ReadOptions {}

struct Table {}

pub struct TableCache {}

impl TableCache {
    /// 从缓存中获取Table
    ///
    /// # Arguments
    ///
    /// * `options`: 读取的配置
    /// * `file_number`: 文件号
    /// * `file_size`: 文件大小
    /// * `k`: key
    /// * `handle_result`: 回调函数
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn get<F>(&self, _options: &ReadOptions, _file_number: u64, _file_size: usize, _k: &Slice, _arg: &mut Saver, _handle_result: F)
        where F: FnMut(&mut Saver, &Slice, &Slice) -> Result<()> {
        ()
    }
    /// 根据文件号消除缓存
    ///
    /// # Arguments
    ///
    /// * `file_number`: 文件号
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn evict(&self, _file_number: u64) {
        todo!()
    }

    /// 获取一个迭代器
    ///
    /// # Arguments
    ///
    /// * `options`: 读取的配置
    /// * `file_number`: 文件号
    /// * `file_size`: 文件大小
    /// * `table`: 表
    ///
    /// returns: Result<dyn DataIterator<Item=<unknown>>, Status>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn new_iterator(&self, _options: &ReadOptions, _file_number: u64, _file_size: usize, _table: &Table) -> Result<Box<dyn DataIterator>> {
        todo!()
    }
}