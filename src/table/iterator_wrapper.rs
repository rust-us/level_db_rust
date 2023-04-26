use crate::traits::DataIterator;
use crate::util::slice::Slice;
use crate::util::status::Status;
use crate::util::unsafe_slice::UnsafeSlice;

pub struct IteratorWrapper {
    iterator: Box<DataIterator>,
    valid: bool,
    key: UnsafeSlice
}

impl IteratorWrapper {
    ///获取原始iterator
    fn iter(&self) -> impl DataIterator {
        return &self.iterator;
    }
    /// 更新iterator
    fn set(&mut self, iter: impl DataIterator) {
        self.iterator = Box::new(iter);
        self.update();
    }
    /// 检查当前位置是否有效
    ///
    /// # Arguments
    ///
    ///
    /// returns: bool
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn valid(&self) -> bool {
        return self.valid;
    }
    /// 获取key
    fn key(&self) -> UnsafeSlice {
        return self.key;
    }
    /// 获取value
    fn value(&self) -> UnsafeSlice {
        return self.iterator.value();
    }

    /// 获取status
    fn status(&self) -> Status {
        return self.iterator.status();
    }

    /// 定位到下一个元素
    ///
    /// # Arguments
    ///
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn next(&mut self) {
        self.iterator.next();
        self.update();
    }

    /// 定位到上一个元素
    ///
    /// # Arguments
    ///
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn prev(&mut self) {
        self.iterator.pre();
        self.update();
    }

    /// 将迭代器定位到指定位置
    ///
    /// # Arguments
    ///
    /// * `target`: 迭代器位置
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn seek(&mut self, target: &Slice) {
        self.iterator.seek(target);
        self.update();
    }

    /// 将迭代器定位到开始位置
    ///
    /// # Arguments
    ///
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn seek_to_first(&mut self) {
        self.iterator.seek_to_first();
        self.update()
    }

    /// 将迭代器定位到末尾位置
    ///
    /// # Arguments
    ///
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn seek_to_last(&mut self) {
        self.iterator.seek_to_last();
        self.update();
    }

    /// 更新迭代器和成员变量
    ///
    /// # Arguments
    ///
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn update(&mut self) {
        self.valid = self.iterator.valid();
        if self.valid == true {
            self.key = self.iterator.key();
        }
    }
}