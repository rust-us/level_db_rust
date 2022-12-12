use std::mem;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::ops::Deref;

pub struct Slice {
    data: Vec<u8>,
}

#[allow(improper_ctypes)]
extern {
    fn memcmp(s1: *const i8, s2: *const i8, n: usize) -> i32;
}

impl Default for Slice {
    /// 构造一个空的 Slice
    fn default() -> Self {
        Self {
            data: Vec::new()
        }
    }
}

impl Slice {
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
}

impl<'a> Slice {
    /// 借取 Slice 中的数据, 调用方只拥有读权限
    pub fn borrow_data(&mut self) -> Cow<'a, String> {
        unsafe {
            // String & Vec<u8> has the same layout
            let s: &String = mem::transmute(&self.data);
            Cow::Borrowed(s)
        }
    }
}

impl From<Slice> for String {
    /// 将 Slice 内数据的所有权移交给 String
    fn from(s: Slice) -> Self {
        unsafe {
            String::from_utf8_unchecked(s.data)
        }
    }
}

impl <R: AsRef<str>> From<R> for Slice {
    fn from(r: R) -> Self {
        Self {
            data: Vec::from(r.as_ref())
        }
    }
}

impl PartialEq for Slice {
    /// 判断两个 Slice 是否相同
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
        match self.size().partial_cmp(&other.size()) {
            Some(Ordering::Equal) => {
                let cmp = unsafe {
                    memcmp(
                        self.data.as_ptr() as *const i8,
                        other.data.as_ptr() as *const i8,
                        self.size(),
                    )
                };
                if cmp == 0 {
                    Some(Ordering::Equal)
                } else if cmp > 0 {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Less)
                }
            }
            op => op
        }
    }
}

impl core::ops::Index<usize> for Slice {
    type Output = u8;

    /// 获取某个下标的数据
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size());
        &(**self)[index]
    }
}

impl Deref for Slice {
    type Target = [u8];

    /// Slice 解引用到 &[u8]
    fn deref(&self) -> &Self::Target {
            &*self.data
    }
}

