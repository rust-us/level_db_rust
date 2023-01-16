use std::io::Write;
use crate::traits::coding_trait::CodingTrait;
use crate::util::coding::Coding;
use crate::util::crc::{AsCrc, CRC};
use crate::util::slice::Slice;
use crate::util::Result;

pub const K_ZERO_TYPE: u8 = 0;
pub const K_FULL_TYPE: u8 = 1;
pub const K_FIRST_TYPE: u8 = 2;
pub const K_MIDDLE_TYPE: u8 = 3;
pub const K_LAST_TYPE: u8 = 4;

pub const K_MAX_RECORD_TYPE: usize = K_LAST_TYPE as usize;
/// Log block size
pub const K_BLOCK_SIZE: usize = 32768;

/// Header is checksum (4 bytes), length (2 bytes), type (1 byte).
pub const K_HEADER_SIZE: usize = 4 + 2 + 1;

const K_EMPTY_BYTES: [u8; 6] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

pub struct LogWriter {
    file_writer: Box<dyn Write>,
    /// Offset in current block
    block_offset: usize,

    type_crc: [u32; K_MAX_RECORD_TYPE + 1],
}

impl LogWriter {
    pub fn new(file_writer: Box<dyn Write>) -> LogWriter {
        let mut type_crc = [0_u32; K_MAX_RECORD_TYPE + 1];
        init_type_crc(&mut type_crc);
        Self {
            file_writer,
            block_offset: 0,
            type_crc,
        }
    }

    pub fn add_record(&mut self, slice: Slice) -> Result<()> {
        let left_over = K_BLOCK_SIZE - self.block_offset;
        assert!(left_over >= 0);
        let mut left = slice.len();
        let mut begin = true;
        let mut start_idx = 0;
        while begin || left > 0 {
            if left_over < K_HEADER_SIZE {
                if left_over > 0 {
                    self.file_writer.write(&K_EMPTY_BYTES[0..left_over])?;
                }
                self.block_offset = 0;
            }
            let avail = K_BLOCK_SIZE - self.block_offset - K_HEADER_SIZE;
            let fragment_length = if left < avail { left } else { avail };
            let end = left == fragment_length;
            let record_type = if begin && end {
                K_FULL_TYPE
            } else if begin {
                K_FIRST_TYPE
            } else if end {
                K_LAST_TYPE
            } else {
                K_MIDDLE_TYPE
            };
            self.emit_physical_record(record_type, slice.as_sub_ref(start_idx, fragment_length))?;
            begin = false;
            left -= fragment_length;
            start_idx += fragment_length;
        }
        Ok(())
    }

    fn emit_physical_record(&mut self, record_type: u8, data: &[u8]) -> Result<()> {
        let mut header = [0_u8; K_HEADER_SIZE];
        header[4] = (data.len() & 0xff) as u8;
        header[5] = (data.len() >> 8) as u8;
        header[6] = record_type;
        let mut crc = CRC::extend(self.type_crc[record_type as usize], data);
        crc = CRC::mask(crc);
        Coding::encode_fixed32(crc, header.as_mut(), 0);
        self.file_writer.write(header.as_ref())?;
        self.block_offset += K_HEADER_SIZE;
        if !data.is_empty() {
            self.file_writer.write(data)?;
            self.block_offset += data.len();
        }
        self.file_writer.flush()?;
        Ok(())
    }
}

fn init_type_crc(type_crc: &mut [u32]) {
    for i in 0..=K_MAX_RECORD_TYPE {
        type_crc[i] = [i as u8].as_crc();
    }
}