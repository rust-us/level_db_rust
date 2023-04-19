mod test {
    use std::fs::File;
    use crate::db::log_reader::LogReader;
    use crate::db::log_writer::LogWriter;
    use crate::util::coding::Decoder;
    use crate::util::crc::{AsCrc, ToMask};
    use crate::util::slice::Slice;
    use crate::util::Result;

    #[test]
    fn write() -> Result<()> {
        let file = Box::new(File::create("../../1.bin")?);
        let mut writer = LogWriter::new(file);
        let sample: Vec<u8> = ('0'..='9').map(|a| a as u8).collect();
        for i in 0..100 {
            let slice = generate_slice(i, &sample);
            writer.add_record(slice)?;
        }
        Ok(())
    }

    #[test]
    fn read() -> Result<()> {
        let file = Box::new(File::open("../../1.bin")?);
        let mut reader = LogReader::new(file, true, 0);
        let sample: Vec<u8> = ('0'..='9').map(|a| a as u8).collect();
        for i in 0..100 {
            let slice = reader.read_next().expect("not error").expect("must have record");
            let expect = generate_slice(i, &sample);
            assert_eq!(expect.len(), slice.len());
            assert_eq!(expect.as_ref(), slice.as_ref())
        }
        Ok(())
    }

    fn generate_slice(i: usize, sample: &Vec<u8>) -> Slice {
        let mut slice = Vec::with_capacity(64);
        for j in 0..=i {
            slice.push(sample[j % 10]);
        }
        Slice::from_vec(slice)
    }

    #[test]
    fn test() {
        let expect_crc_bytes: [u8; 4] = [0xD1, 0xB1, 0x09, 0x9A];
        let mut decoder = Decoder::with_buf(&expect_crc_bytes);
        let expect_crc = unsafe { decoder.uncheck_get_fixed32() };
        let raw_bytes = [0x01_u8, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
            0x39, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39];
        let crc = raw_bytes.as_crc().to_mask();
        let partial_extend = raw_bytes[0..1].as_crc();
        let crc1 = raw_bytes[1..].as_crc_extend(partial_extend).to_mask();
        println!("expect_crc: {}, crc: {}, crc1: {}", expect_crc, crc, crc1);
        assert_eq!(expect_crc, crc);
        assert_eq!(expect_crc, crc1);
    }
}