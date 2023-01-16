use std::fs::File;
use std::io::{Read, Seek, Write};
use std::io::SeekFrom::Start;

use crate::db::log_writer::{K_BLOCK_SIZE, K_FIRST_TYPE, K_FULL_TYPE, K_LAST_TYPE, K_MIDDLE_TYPE};
use crate::traits::coding_trait::CodingTrait;
use crate::util::coding::Coding;
use crate::util::crc::{AsCrc, CRC};
use crate::util::Result;
use crate::util::slice::Slice;
use crate::util::status::{LevelError, Status};

pub struct LogReader {
    file_reader: Box<dyn SeekableReader>,
    checksum: bool,
    read_pos: usize,
    eof: bool,
    buf: [u8; K_BLOCK_SIZE],
    buf_len: usize,
    buf_read_idx: usize,
}

pub trait SeekableReader: Read + Seek {}
// pub type SeekableReader = dyn Read + Seek;

impl SeekableReader for File {}

impl LogReader {
    pub fn new(mut file_reader: Box<dyn SeekableReader>, checksum: bool, initial_offset: usize) -> LogReader {
        let offset_in_block = initial_offset % K_BLOCK_SIZE;
        let mut block_start_location = initial_offset - offset_in_block;
        if offset_in_block > K_BLOCK_SIZE - 6 {
            block_start_location += K_BLOCK_SIZE;
        }
        file_reader.seek(Start(block_start_location as u64)).expect("seek to initial_offset");
        Self {
            file_reader,
            checksum,
            read_pos: block_start_location,
            eof: false,
            buf: [0; K_BLOCK_SIZE],
            buf_len: 0,
            buf_read_idx: 0,
        }
    }

    pub fn read_next(&mut self) -> Result<Option<Slice>> {
        let mut tmp: Option<Vec<u8>> = None;
        loop {
            if self.buf_read_idx >= self.buf_len {
                self.read_buf()?;
            }
            let data_len = (self.buf[self.buf_read_idx + 4] as usize) + ((self.buf[self.buf_read_idx + 5] as usize) << 8);
            let record_type = self.buf[self.buf_read_idx + 6];
            self.buf_read_idx += 7;
            // CRC check
            self.check_crc(data_len)?;
            match record_type {
                K_FULL_TYPE => {
                    let end_idx = self.buf_read_idx + data_len;
                    let data = &self.buf[self.buf_read_idx..end_idx];
                    self.buf_read_idx += data_len;
                    return Ok(Some(Slice::from_buf(data)));
                }
                K_FIRST_TYPE => {
                    tmp = Some(Vec::with_capacity(K_BLOCK_SIZE));
                    let partial_data = &self.buf[self.buf_read_idx..];
                    tmp.as_mut().unwrap().write(partial_data)?;
                }
                K_MIDDLE_TYPE => {
                    tmp.as_mut().unwrap().write(self.buf.as_ref())?;
                }
                K_LAST_TYPE => {
                    let end_idx = self.buf_read_idx + data_len;
                    let partial_data = &self.buf[self.buf_read_idx..end_idx];
                    self.buf_read_idx += data_len;
                    tmp.as_mut().unwrap().write(partial_data)?;
                    let data = tmp.unwrap();
                    return Ok(Some(Slice::from_vec(data)));
                }
                _ => {
                    return Err(Status::wrapper(LevelError::KCorruption,
                                               format!("bad record_type: {}", record_type).into()));
                }
            }
        }
    }

    fn read_buf(&mut self) -> Result<()> {
        self.buf_read_idx = 0;
        self.buf_len = self.file_reader.read(self.buf.as_mut())?;
        if self.buf_len < K_BLOCK_SIZE {
            self.eof = true;
        }
        Ok(())
    }

    #[inline]
    fn check_crc(&self, data_len: usize) -> Result<()> {
        if !self.checksum {
            return Ok(());
        }
        let crc_bytes = &self.buf[(self.buf_read_idx - 7)..(self.buf_read_idx - 3)];
        let expect = Coding::decode_fixed32(crc_bytes);
        let data = &self.buf[(self.buf_read_idx - 1)..(self.buf_read_idx + data_len)];
        let crc = data.as_crc();
        let mask = CRC::mask(crc);
        if expect == mask {
            Ok(())
        } else {
            Err(Status::wrapper(LevelError::KCorruption, "bad record, crc check failed".into()))
        }
    }
}