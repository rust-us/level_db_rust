use crate::traits::coding_trait::CodingTrait;
use crate::util::coding;
use crate::util::slice::Slice;
use crate::util::Result;

/// Maximum encoding length of a BlockHandle
pub const k_max_encoded_length: u32 = 10 + 10;

// // kTableMagicNumber was picked by running
// //    echo http://code.google.com/p/leveldb/ | sha1sum
// // and taking the leading 64 bits.
// pub const k_table_magic_number: &str = 0xdb4775248b80fb57ull;

pub struct BlockHandle {
    // 偏移量
    offset_: u64,
    //
    size_: u64
}

/// Footer encapsulates the fixed information stored at the tail
/// end of every table file.
pub struct Footer {
    metaindex_handle_: BlockHandle,
    index_handle_: BlockHandle
}

trait Block {
    ///
    /// The offset of the block in the file.
    ///
    fn offset(&self) -> u64;

    ///
    /// set offset
    /// # Arguments
    ///
    /// * `offset`:
    ///
    fn set_offset(&mut self, offset: u64);

    // The size of the stored block
    fn size(&self) -> u64;

    ///
    /// set size
    /// # Arguments
    ///
    /// * `size`:
    ///
    fn set_size(&mut self, size: u64);

    ///
    /// 将 Block 对象编码成 Slice
    ///
    /// # Arguments
    ///
    /// returns: Slice
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn encode_to(&self) -> Slice;

    ///
    /// 将 Slice 对象解码后与BlockHandle比较，是否可以成功
    ///
    /// # Arguments
    /// * `input`:
    ///
    /// returns: Result
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn decode_from(&self, input: Slice) -> Result<bool>;
}

trait Foot {

}

impl Block for BlockHandle {
    fn offset(&self) -> u64 {
        self.offset_
    }

    fn set_offset(&mut self, offset: u64) {
        self.offset_ = offset;
    }

    fn size(&self) -> u64 {
        self.size_
    }

    fn set_size(&mut self, size: u64) {
        self.size_ = size;
    }

    fn encode_to(&self) -> Slice {
        todo!()

        // // Sanity check that all fields have been set
        // assert!(self.offset_ != 0);
        // assert!(self.size_ != 0);
        //
        // let mut buf: [u8; 4] = [0, 0, 0, 0];
        // coding::Coding::put_varint64(&mut buf, 0, &self.offset_);
        //
        // Slice::default()
    }

    fn decode_from(&self, input: Slice) -> Result<String> {
        todo!()
    }
}