use std::slice;
use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;
use std::sync::{Arc, Mutex};

use crate::util::slice::Slice;

// Arena block size
const ARENA_BLOCK_SIZE: usize = 4096;

pub type ArenaRef = Arc<Mutex<Arena>>;


///
pub trait ArenaAllocLike {
    fn copy_with_arena(&self, arena: ArenaRef) -> Self;
}

pub struct Arena {
    alloc_ptr: Option<NonNull<u8>>,
    alloc_bytes_remaining: usize,
    blocks: Vec<(NonNull<u8>, Layout)>,
    memory_usage: usize,
}

impl Default for Arena {
    fn default() -> Self {
        Self {
            alloc_ptr: None,
            alloc_bytes_remaining: 0,
            blocks: vec![],
            memory_usage: 0,
        }
    }
}

impl Arena {
    /// 申请一块内存
    ///
    /// # Arguments
    ///
    /// * `bytes`: 申请内存大小(byte)
    ///
    /// returns: &mut [u8]
    /// 内存的 byte 数组
    /// # Examples
    ///
    /// ```
    ///let arena = Arena::default();
    /// // 申请 12 字节大小的内存
    /// let buf = arena.allocate(12);
    /// ```
    #[inline]
    pub fn allocate(&mut self, bytes: usize) -> &mut [u8] {
        self.allocate_align(bytes, 1)
    }

    pub fn allocate_align(&mut self, bytes: usize, align: usize) -> &mut [u8] {
        if bytes <= self.alloc_bytes_remaining {
            self.alloc_bytes_remaining -= bytes;
            let result = unsafe { slice::from_raw_parts_mut(self.alloc_ptr.unwrap().as_ptr(), bytes) };
            unsafe {
                let new_ptr = self.alloc_ptr.unwrap().as_ptr().offset(bytes as isize);
                self.alloc_ptr = Some(NonNull::new_unchecked(new_ptr));
            };
            return result;
        }
        return self.allocate_fallback(bytes, align);
    }

    #[inline]
    pub fn memory_usage(&self) -> usize {
        self.memory_usage
    }

    fn allocate_fallback(&mut self, bytes: usize, align: usize) -> &mut [u8] {
        if bytes > ARENA_BLOCK_SIZE / 4 {
            unsafe {
                let layout = Layout::from_size_align_unchecked(bytes, align);
                return self.allocate_new_block(layout);
            }
        }
        unsafe {
            self.alloc_bytes_remaining = ARENA_BLOCK_SIZE - bytes;
            let layout = Layout::from_size_align_unchecked(ARENA_BLOCK_SIZE, align);
            let new_block = self.allocate_new_block(layout);
            let ptr = new_block.as_ptr() as *mut u8;
            let result = slice::from_raw_parts_mut(ptr, bytes);
            self.alloc_ptr = Some(NonNull::new_unchecked(ptr.offset(bytes as isize)));
            result
        }
    }


    /// 分配一块新的内存
    fn allocate_new_block(&mut self, layout: Layout) -> &mut [u8] {
        unsafe {
            let data = alloc(layout);
            self.memory_usage += layout.size();
            self.blocks.push((NonNull::new_unchecked(data), layout));
            slice::from_raw_parts_mut(data, layout.size())
        }
    }
}

impl Drop for Arena {
    /// 释放内存
    fn drop(&mut self) {
        for (block, layout) in self.blocks.iter() {
            unsafe {
                dealloc(block.as_ptr(), *layout)
            }
        }
    }
}

impl ArenaAllocLike for Slice {
    fn copy_with_arena(&self, arena: ArenaRef) -> Self {
        unsafe {
            let mut lock_guard = arena.lock().unwrap();
            let dst = lock_guard.allocate(self.len());
            let src = &**self;
            dst.copy_from_slice(src);
            Slice::from_raw_parts(dst.as_mut_ptr(), self.len())
        }
    }
}