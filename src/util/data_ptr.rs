use std::mem;
use std::mem::size_of;

#[derive(Copy, Clone)]
pub struct DataPtr {
    ptr: *mut u8,
}

impl From<*mut u8> for DataPtr {
    #[inline]
    fn from(ptr: *mut u8) -> Self {
        Self {
            ptr
        }
    }
}

impl From<*const u8> for DataPtr {
    #[inline]
    fn from(ptr: *const u8) -> Self {
        Self {
            ptr: ptr as *mut u8
        }
    }
}

impl DataPtr {
    /// 向 DataPtr 中写入一个值
    ///
    /// # Unsafe
    /// 调用方需要保证写入数据的大小不会超过 DataPtr 剩余可用的内存大小
    ///
    /// # Arguments
    ///
    /// * `value`: 要被写入的值
    ///
    /// returns: DataPtr 写入后的指针偏移量
    ///
    /// # Examples
    ///
    /// ```
    /// use std::alloc::{alloc, Layout};
    /// let raw_ptr = unsafe {
    ///     alloc(Layout::from_size_align(8, 8).unwrap())
    /// };
    /// let ptr = DataPtr::from(raw_ptr);
    /// unsafe {
    ///     // write multi values
    ///     ptr.write(18_u32).write(32_u32);
    /// }
    /// ```
    #[inline]
    pub unsafe fn write<T>(&self, value: T) -> DataPtr
        where T: Sized {
        unsafe {
            (self.ptr as *mut _ as *mut T).write(value);
            self.ptr.offset(size_of::<T>() as isize).into()
        }
    }


    /// 让 DataPtr 的指针偏移指定长度
    ///
    /// # Unsafe
    /// 调用方必须保证增加额外得偏移长度不会造成指针越界
    ///
    /// # Arguments
    ///
    /// * `offset`: 偏移长度
    ///
    /// returns: DataPtr
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    #[inline]
    pub unsafe fn offset(&self, offset: isize) -> DataPtr {
        self.ptr.offset(offset).into()
    }


}