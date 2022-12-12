use std::mem::size_of;

use crate::util::data_ptr::DataPtr;

/// 编解码特性
/// LevelDB需要支持的基本类型都需要实现此特质, 如 u8, i8, u32 等
pub trait Coding<T: Sized> {

    fn write(self, ptr: DataPtr) -> DataPtr;

    fn read(ptr: DataPtr) -> T;

    fn read_once(ptr: DataPtr) -> (T, DataPtr) {
        let value: T = Self::read(ptr);
        unsafe {
            (value, ptr.offset(size_of::<T>() as isize))
        }
    }
}