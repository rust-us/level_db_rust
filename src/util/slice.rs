use std::alloc::{alloc, Layout};
use std::borrow::{Borrow, Cow};
use std::ptr;
use std::ptr::{copy, NonNull};

pub struct Slice {
    data: NonNull<u8>,
    len: usize,
}

#[allow(improper_ctypes)]
extern {
    fn memcmp(s1: *const i8, s2: *const i8, n: usize) -> i32;
}

impl Default for Slice {
    /// 构造一个空的 Slice
    fn default() -> Self {
        unsafe {
            Self {
                data: NonNull::new_unchecked(ptr::null_mut()),
                len: 0,
            }
        }
    }
}

impl Slice {
    /// 获取 slice 长度
    #[inline]
    pub fn size(&self) -> usize {
        self.len
    }

    /// 判断 slice 是否为空
    #[inline]
    pub fn empty(&self) -> bool {
        self.len == 0
    }

    /// 移除头部 n 个元素
    pub fn remove_prefix(&self, n: usize) -> Slice {
        assert!(self.len >= n);
        if self.len == 0 {
            return Slice::default();
        }
        let len = self.len - n;
        unsafe {
            let data = alloc(Layout::array(len).unwrap());
            copy(self.data.as_ptr().offset(n as isize), data, len);
            Self {
                data: NonNull::new_unchecked(data),
                len,
            }
        }
    }

    /// 判断本 Slice 是否以 other 为开始
    pub fn starts_with(&self, other: &Self) -> bool {
        assert!(other.len <= self.len);
        if other.len == 0 {
            return true;
        }
        return self.len >= other.len && unsafe {
            memcmp(
                self.data.as_ptr() as *const i8,
                other.data.as_ptr() as *const i8,
                other.len) == 0
        };
    }
}

impl<'a> Slice {
    /// 借取 Slice 中的数据, 调用方只拥有读权限
    pub fn borrow_data(&self) -> Cow<'a, String> {
        let str = unsafe {
            String::from_raw_parts(self.data.as_ptr(), self.len, self.len)
        };
        Cow::Borrowed(&str)
    }
}

impl Into<String> for Slice {
    /// 将 Slice 内数据的所有权移交给 String
    fn into(self) -> String {
        unsafe {
            String::from_raw_parts(self.data.as_ptr(), self.len, self.len)
        }
    }
}

impl Into<Slice> for String {
    /// 通过 String 构造一个 Slice
    fn into(mut self) -> Slice {
        unsafe {
            Slice {
                data: NonNull::new_unchecked(self.as_mut_ptr()),
                len: str.len(),
            }
        }
    }
}

impl Into<Slice> for &str {
    /// 通过 &str 构造一个 Slice
    fn into(mut self) -> Slice {
        unsafe {
            Slice {
                data: NonNull::new_unchecked(self.as_mut_ptr()),
                len: slice.len(),
            }
        }
    }
}

impl PartialEq for Slice {
    /// 判断两个 Slice 是否相同
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl core::ops::Index<isize> for Slice {
    type Output = u8;

    /// 获取某个下标的数据
    fn index(&self, index: isize) -> &Self::Output {
        if index < self.len as isize {
            unsafe {
                &self.data.as_ptr().offset(index).read()
            }
        } else {
            &0_u8
        }
    }
}

impl Drop for Slice {
    /// 释放内存
    fn drop(&mut self) {
        let _: String = self.into();
    }
}

