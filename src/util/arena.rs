use std::ptr::NonNull;
use crate::util::data_ptr::DataPtr;

// Arena block size
const ARENA_BLOCK_SIZE: usize = 4096;

struct Arena {

}

impl Arena {

    pub fn allocate(_bytes: usize) -> NonNull<u8> {
        todo!()
    }

    pub fn allocate_align(_bytes: usize) -> NonNull<u8> {
        todo!()
    }

    fn allocate_fallback(_bytes: usize) -> DataPtr {
        todo!()
    }

    fn allocate_new_block(_block_size: usize) -> DataPtr {
        todo!()
    }

}

impl Drop for Arena {

    fn drop(&mut self) {
        todo!()
    }
}