use std::alloc::{alloc, Layout};
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::mem::ManuallyDrop;

use crate::util::arena::ArenaRef;
use crate::util::Result;
use crate::util::slice::Slice;

/// 提供一种将其它结构体转为 UnsafeSlice 的特质
pub trait TryIntoUnsafeSlice {
    /// 尝试将结构体通过 arena 内存分配器，构造出一个新的 UnsafeSlice
    fn try_into_unsafe_slice(&self, arena: ArenaRef) -> Result<UnsafeSlice>;
}

/// 内存不安全的 Slice, 内存由 Arena 分配和管理。
/// 实现了 Copy 语义，有更高效的读 api
#[derive(Copy, Clone)]
pub struct UnsafeSlice {
    ptr: *mut u8,
    len: usize,
}

impl UnsafeSlice {

    /// 利用 arena 生成 UnsafeSlice
    pub fn new_with_arena<B: AsRef<[u8]>>(data: B, arena: ArenaRef) -> Result<Self> {
        let mut lock = arena.lock()?;
        let src = data.as_ref();
        let mut buf = lock.allocate(src.len());
        buf.write(src)?;
        Ok(Self {
            ptr: buf.as_mut_ptr(),
            len: buf.len(),
        })
    }



    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(self.as_ref())
        }
    }
}

impl UnsafeSlice {

    /// 返回子串。这个方法是高效的，在内部只复制了裸指针偏的移量。
    pub unsafe fn sub_slice(&self, start: usize, len: usize) -> Self {
        assert!(start + len < self.len, "sub_slice out of range");
        Self {
            ptr: self.ptr.offset(start as isize),
            len,
        }
    }

    /// 生成 Slice 串，由于 Slice 是内存安全的，所以实现上会有内存拷贝。
    /// 高性能场景优先考虑 UnsafeSlice
    pub fn to_slice(&self) -> Slice {
        unsafe {
            let raw_ptr = alloc(Layout::from_size_align_unchecked(self.len, 8));
            Slice::from_raw_parts(raw_ptr, self.len)
        }
    }
}

impl AsRef<[u8]> for UnsafeSlice {

    #[inline]
    fn as_ref(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(self.ptr, self.len)
        }
    }
}

impl Display for UnsafeSlice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let string = ManuallyDrop::new(
                String::from_raw_parts(self.ptr, self.len, self.len)
            );
            f.write_str(string.as_str())
        }
    }
}

impl TryIntoUnsafeSlice for &str {
    #[inline]
    fn try_into_unsafe_slice(&self, arena: ArenaRef) -> Result<UnsafeSlice> {
        UnsafeSlice::new_with_arena(self.as_bytes(), arena)
    }
}

impl TryIntoUnsafeSlice for String {
    #[inline]
    fn try_into_unsafe_slice(&self, arena: ArenaRef) -> Result<UnsafeSlice> {
        UnsafeSlice::new_with_arena(self.as_bytes(), arena)
    }
}