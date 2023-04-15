use crate::util::slice::Slice;
use crate::util::Result;
use crate::util::status::Status;

/// Maximum encoding length of a BlockHandle
pub const k_max_encoded_length: u32 = 10 + 10;

/// Encoded length of a Footer.  Note that the serialization of a
/// Footer will always occupy exactly this many bytes.  It consists
/// of two block handles and a magic number.
pub const k_encoded_length: u32 = 2 * k_max_encoded_length + 8;

/// kTableMagicNumber was picked by running echo http://code.google.com/p/leveldb/ | sha1sum and taking the leading 64 bits.
pub const k_table_magic_number: u64 = 0xdb4775248b80fb57;

/// 1-byte type + 32-bit crc
pub const k_block_trailer_size: usize = 5;

pub struct BlockHandle {
    // 偏移量， 编码为可变长度的64位整列，最多占用10个字节
    offset: u64,
    // 大小， 编码为可变长度的64位整列，最多占用10个字节
    size: u64
}

trait ToBlockHandle {
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
    fn encode_to(&self) -> Result<Slice>;

    ///
    /// 将 Slice 对象解码后与BlockHandle set field
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
    fn decode_from(&mut self, input: Slice) -> Result<()>;
}

/// Footer 的大小为 48 字节，最后8个字节为 magic number， 通过魔术对比，可以判断一个文件是否为 SST 文件。
/// 其余40个字节由三部分构成：
///     1、前两个部分是两个 BlockHandle。BlockHandle 中主要包括两个变量：偏移量offset，大小size。
///     通过这两个 BlockHandle 可以分别定位到数据索引区域（data block index）以及元数据索引区域（meta block index）.
///     2、 由于 BlockHandle 的成员变量使用可变长度编码，每个 BlockHandle 最大占用20字节，
///     因此如果前两部分不足40字节，则需要padding结构补充，这也构成了第三部分。
///  PS: 可变长度编码 变长的64位整型。
///
pub struct Footer {
    meta_index_handle: BlockHandle,
    index_handle: BlockHandle
}

trait ToFoot {
    // The block handle for the metaindex block of the table
    fn meta_index_handle(&self) -> BlockHandle;

    fn set_metaindex_handle(&mut self, h: BlockHandle);

    fn index_handle(&self) -> BlockHandle;

    fn set_index_handle(&mut self, h: BlockHandle);

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
    fn encode_to(&self) -> Result<Slice>;

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
    fn decode_from(&mut self, input: Slice) -> Result<()>;
}

impl ToBlockHandle for BlockHandle {
    fn offset(&self) -> u64 {
        self.offset
    }

    fn set_offset(&mut self, offset: u64) {
        self.offset = offset;
    }

    fn size(&self) -> u64 {
        self.size
    }

    fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    fn encode_to(&self) -> Result<Slice> {
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

    fn decode_from(&mut self, input: Slice) -> Result<()> {
        todo!()
    }
}

impl Default for BlockHandle {
    #[inline]
    fn default() -> Self {
        BlockHandle {
            offset: 0,
            size: 0,
        }
    }
}

impl ToFoot for Footer {
    /// The block handle for the metaindex block of the table
    fn meta_index_handle(&self) -> BlockHandle {
        todo!()
    }

    fn set_metaindex_handle(&mut self, h: BlockHandle) {
        todo!()
    }

    fn index_handle(&self) -> BlockHandle {
        todo!()
    }

    fn set_index_handle(&mut self, h: BlockHandle) {
        todo!()
    }

    fn encode_to(&self) -> Result<Slice> {
        todo!()
    }

    fn decode_from(&mut self, input: Slice) -> Result<()> {
        todo!()
    }
}

/// ############################# BlockContent
pub struct BlockContent {
    // Actual contents of data
    data: Slice,

    // True if data can be cached
    cachable: bool,

    // True if caller should delete[] data.data()
    heap_allocated:bool,
}

trait ToBlockContent {
    /// Read the block identified by "handle" from "file".  On failure
    /// return non-OK.  On success fill *result and return OK.
    fn read_block(&self,
                  // todo RandomAccessFile, ReadOptions 未提供
                  // file: RandomAccessFile, options: ReadOptions,
                  handle: BlockHandle
    ) -> Result<BlockContent>;

}

impl ToBlockContent for BlockContent {
    fn read_block(&self, handle: BlockHandle) -> Result<BlockContent> {
        todo!()
    }
}
