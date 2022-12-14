use crate::util::crc::CRC;

#[test]
fn test_crc() {
    // From rfc3720 section B.4.
    let mut buf = [0_u8; 32];
    let crc0 = CRC::value(&buf);
    assert_eq!(0x8a9136aa, crc0);
    // println!("crc0: {:x}, eq: {:x}", crc0, 0x8a9136aa_u32);
    buf.fill(0xff);
    assert_eq!(0x62a8ab43, CRC::value(&buf));
    (0..32).for_each(|idx| buf[idx] = idx as u8);
    assert_eq!(0x46dd794e, CRC::value(&buf));
    (0..32).for_each(|idx| buf[idx] = 31-(idx as u8));
    assert_eq!(0x113fdb5c, CRC::value(&buf));
    let data = [
        0x01_u8, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00,
        0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x18, 0x28, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    ];
    assert_eq!(0xd9963a56, CRC::value(&data));
}

#[test]
fn test_extend() {
    let init_crc = CRC::value("hello ".as_bytes());
    let crc = CRC::extend(init_crc, "world".as_bytes());
    let value = CRC::value("hello world".as_bytes());
    assert_eq!(value, crc);
}

#[test]
fn test_mask() {
    let crc = CRC::value("foo".as_bytes());
    assert_ne!(crc, CRC::mask(crc));
    assert_ne!(crc, CRC::mask(CRC::mask(crc)));
    assert_eq!(crc, CRC::unmask(CRC::mask(crc)));
    assert_eq!(crc, CRC::unmask(CRC::unmask(CRC::mask(CRC::mask(crc)))));
}