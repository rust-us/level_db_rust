use std::cell::RefCell;
use std::rc::Rc;
use crate::util::slice::Slice;

use crate::util::Result;

struct Handle {}

pub struct LRUHandle<T> {
    key: Slice,
    value: T,
    hash: u32,
    in_cache: bool,
    key_length: usize,
    charge: usize,
    prev: Option<Rc<RefCell<LRUHandle<T>>>>,
    next: Option<Rc<RefCell<LRUHandle<T>>>>,
    next_hash: Option<Rc<RefCell<u32>>>,
}

impl<T> LRUHandle<T> {
    pub fn key(&self) -> Slice {
        todo!()
    }
}

pub struct HandleTable {
    length: usize,
}

impl HandleTable {
    pub fn look_up<T>(&self, _key: &Slice, _hash: u32) -> &LRUHandle<T> {
        todo!()
    }

    pub fn insert<T>(&mut self, _handle: LRUHandle<T>) -> &LRUHandle<T> {
        todo!()
    }

    pub fn remove<T>(&mut self, _key: &Slice, _hash: u32) -> LRUHandle<T> {
        todo!()
    }

    pub fn length(&self) -> usize {
        self.length
    }

    fn resize(&mut self) {
        todo!()
    }
}

pub struct LRUCache<T> {
    capacity: usize,
    usage: usize,
    in_use: LRUHandle<T>,
    table: HandleTable,
}

impl<T> LRUCache<T> {
    pub fn set_capacity(&mut self, capacity: usize) {
        self.capacity = capacity;
    }

    pub fn insert(&mut self, _key: &Slice, _hash: u32, _value: T, _charge: usize) -> &LRUHandle<T> {
        todo!()
    }

    pub fn look_up(&self, _key: &Slice, _hash: u32) -> &LRUHandle<T> {
        todo!()
    }

    pub fn release(&mut self, _handle: &LRUHandle<T>) {
        todo!()
    }

    pub fn erase(&mut self, _key: &Slice, _hash: u32) {
        todo!()
    }
    pub fn prune(&mut self) {
        todo!()
    }
    pub fn total_charge(&self) -> usize {
        todo!()
    }

    pub fn lru_remove(&mut self, _handle: &LRUHandle<T>) {
        todo!()
    }
    pub fn lru_append(&mut self, _head_of_list: &LRUHandle<T>, _e: LRUHandle<T>) {
        todo!()
    }
    pub fn refer(&self, _e: &LRUHandle<T>) {
        todo!()
    }
    pub fn unref(&self, _e: &LRUHandle<T>) {
        todo!()
    }
}

pub trait Cache<T> {
    /// 向缓存中插入数据
    ///
    /// # Arguments
    ///
    /// * `key`: 键
    /// * `value`: 值
    /// * `charge`: 长度
    /// * `deleter`: 删除的回调函数
    ///
    /// returns: Handle
    ///
    /// # Examples
    ///
    /// ```
    /// let element = cache.insert(Slice::from("123"), block, 10, move || {});
    /// ```
    fn insert<F>(&mut self, key: &Slice, value: T, charge: usize, deleter: F) -> Handle
        where F: FnOnce(&Slice, T);

    /// 从缓存中读取数据
    ///
    /// # Arguments
    ///
    /// * `key`: 键
    ///
    /// returns: Handle
    ///
    /// # Examples
    ///
    /// ```
    /// let element = cache.lookup(Slice::from("123"));
    /// ```
    fn lookup(&self, key: &Slice) -> Handle;

    /// 从缓存中释放元素
    ///
    /// # Arguments
    ///
    /// * `handle`: 元素
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// cache.release(element);
    /// ```
    fn release(&mut self, handle: Handle);

    /// 从缓存中删除元素
    ///
    /// # Arguments
    ///
    /// * `key`: 键
    ///
    /// returns: Result<(), Status>
    ///
    /// # Examples
    ///
    /// ```
    /// cache.erase(Slice::from("123"))?;
    /// ```
    fn erase(&mut self, key: &Slice) -> Result<()>;

    fn new_id(&self) -> Result<u64>;
    fn prune(&mut self) -> Result<()>;
    fn total_charge(&self) -> usize;
    // fn value(&self, key: Handle) -> Handle;
}