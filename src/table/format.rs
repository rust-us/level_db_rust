use crate::traits::coding_trait::CodingTrait;
use crate::util::coding;
use crate::util::slice::Slice;
use crate::util::Result;

/// Maximum encoding length of a BlockHandle
pub const k_max_encoded_length: u32 = 10 + 10;

/// Encoded length of a Footer.  Note that the serialization of a
/// Footer will always occupy exactly this many bytes.  It consists
/// of two block handles and a magic number.
pub const k_encoded_length: u32 = 2 * k_max_encoded_length + 8;

// // kTableMagicNumber was picked by running
// //    echo http://code.google.com/p/leveldb/ | sha1sum
// // and taking the leading 64 bits.
// pub const k_table_magic_number: &str = 0xdb4775248b80fb57ull;

/// 1-byte type + 32-bit crc
pub const k_block_trailer_size: usize = 5;

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

pub struct BlockContents {
    // Actual contents of data
    data: Slice,

    // True if data can be cached
    cachable: bool,

    // True if caller should delete[] data.data()
    heap_allocated:bool,
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
    // The block handle for the metaindex block of the table
    fn metaindex_handle() -> BlockHandle;

    fn set_metaindex_handle(h: BlockHandle);

    fn index_handle() -> BlockHandle;

    fn set_index_handle(h: BlockHandle);

    ///
    /// 将 Foot 对象编码成 Slice
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
    /// 将 Slice 对象解码后与 BlockHandle 比较，是否可以成功
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

trait BlockContent {
    /// Read the block identified by "handle" from "file".  On failure
    /// return non-OK.  On success fill *result and return OK.
    fn read_block(
        // todo RandomAccessFile, ReadOptions 未提供
        // file: RandomAccessFile, options: ReadOptions,
        handle: BlockHandle
    ) -> Result<BlockContents>;

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

    fn decode_from(&self, input: Slice) -> Result<bool> {
        todo!()
    }
}

impl Default for BlockHandle {
    #[inline]
    fn default() -> Self {
        BlockHandle {
            offset_: 0,
            size_: 0,
        }
    }
}

impl Foot for Footer {
    fn metaindex_handle() -> BlockHandle {
        todo!()
    }

    fn set_metaindex_handle(h: BlockHandle) {
        todo!()
    }

    fn index_handle() -> BlockHandle {
        todo!()
    }

    fn set_index_handle(h: BlockHandle) {
        todo!()
    }

    fn encode_to(&self) -> Slice {
        todo!()
    }

    fn decode_from(&self, input: Slice) -> Result<bool> {
        todo!()
    }
}

impl BlockContent for BlockContents {
    fn read_block(handle: BlockHandle) -> Result<BlockContents> {
        todo!()
    }
}
