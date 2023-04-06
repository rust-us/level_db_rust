use std::mem;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::mem::ManuallyDrop;
use std::ops::Deref;

#[derive(Debug)]
pub struct Slice {
    data: Vec<u8>,
}

#[allow(improper_ctypes)]
extern {
    fn memcmp(s1: *const i8, s2: *const i8, n: usize) -> i32;
}

impl Default for Slice {
    /// 构造一个空的 Slice
    #[inline]
    fn default() -> Self {
        Self {
            data: Vec::new()
        }
    }
}

impl Slice {

    /// 从 &mut [u8] 转到 Slice, 这里存在内存拷贝开销
    #[inline]
    pub fn from_buf(buf: &[u8]) -> Self {
        Self {
            data: buf.to_owned()
        }
    }

    #[inline]
    pub fn from_vec(data: Vec<u8>) -> Self {
        Self {
            data
        }
    }

    #[inline]
    pub unsafe fn from_raw_parts(ptr: *mut u8, len: usize) -> Self {
        let data = Vec::from_raw_parts(ptr, len, len);
        Self { data }
    }

    /// 获取 slice 长度
    #[inline]
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// 判断 slice 是否为空
    #[inline]
    pub fn empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    pub fn as_sub_ref(&self, start: usize, length: usize) -> &[u8] {
        &(**self)[start..(start+length)]
    }

    /// 移除头部 n 个元素
    pub fn remove_prefix(&self, n: usize) -> Slice {
        assert!(self.size() >= n);
        if self.size() == 0 {
            return Slice::default();
        }
        let sub_data = &(*self.data)[n..self.size()];
        Self {
            data: Vec::from(sub_data)
        }
    }

    /// 判断本 Slice 是否以 other 为开始
    pub fn starts_with(&self, other: &Self) -> bool {
        assert!(other.size() <= self.size());
        if other.size() == 0 {
            return true;
        }
        return self.size() >= other.size() && unsafe {
            memcmp(
                self.data.as_ptr() as *const i8,
                other.data.as_ptr() as *const i8,
                other.size()) == 0
        };
    }

    pub fn merge(&mut self, mut other: Self, joiner: Option<String>) {
        if other.empty() {
            return;
        }
        match joiner {
            None => self.data.append(&mut other.data),
            Some(mut j) => unsafe {
                self.data.append(j.as_mut_vec());
                self.data.append(&mut other.data);
            }
        }
    }

    pub fn as_str(&self) -> &str {
        let s = self.as_ref();
        std::str::from_utf8(s).unwrap()
    }
}

impl<'a> Slice {
    /// 借取 Slice 中的数据, 调用方只拥有读权限
    pub fn borrow_data(&self) -> Cow<'a, String> {
        unsafe {
            // String & Vec<u8> has the same layout
            let s: &String = mem::transmute(&self.data);
            Cow::Borrowed(s)
        }
    }
}

impl Clone for Slice {
    fn clone(&self) -> Self {
        let data = self.data.clone();
        Slice::from_vec(data)
    }
}

impl From<Slice> for String {
    /// 将 Slice 内数据的所有权移交给 String
    #[inline]
    fn from(s: Slice) -> Self {
        unsafe {
            String::from_utf8_unchecked(s.data)
        }
    }
}

impl From<Slice> for Vec<u8> {
    #[inline]
    fn from(s: Slice) -> Self {
        s.data
    }
}

impl <R: AsRef<str>> From<R> for Slice {
    #[inline]
    fn from(r: R) -> Self {
        Self {
            data: Vec::from(r.as_ref())
        }
    }
}

impl AsRef<[u8]> for Slice {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.data.as_slice()
    }
}

impl PartialEq for Slice {
    /// 判断两个 Slice 是否相同
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        return self.size() == other.size() && unsafe {
            memcmp(
                self.data.as_ptr() as *const i8,
                other.data.as_ptr() as *const i8,
                self.size(),
            ) == 0
        };
    }
}

impl PartialOrd for Slice {
    /// 判断两个 slice 的大小关系
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let min = self.size().min(other.size());
        let cmp = unsafe {
            memcmp(
                self.data.as_ptr() as *const i8,
                other.data.as_ptr() as *const i8,
                min,
            )
        };
        if cmp == 0 {
            self.size().partial_cmp(&other.size())
        } else if cmp > 0 {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl core::ops::Index<usize> for Slice {
    type Output = u8;

    /// 获取某个下标的数据
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size());
        &(**self)[index]
    }
}

impl Deref for Slice {
    type Target = [u8];

    /// Slice 解引用到 &[u8]
    #[inline]
    fn deref(&self) -> &Self::Target {
            &*self.data
    }
}

impl Display for Slice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let string = ManuallyDrop::new(
                String::from_raw_parts(
                    self.as_ptr() as *mut u8,
                    self.data.len(),
                    self.data.capacity())
            );
            f.write_str(string.as_str())
        }
    }
}
