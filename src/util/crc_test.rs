use crate::util::crc::{AsCrc, CRC, ToMask};
use crate::util::slice::Slice;

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
fn test_as_crc() {
    let expect = CRC::value("123".as_bytes());
    let crc0 = "123".as_crc();
    assert_eq!(expect, crc0);
    let crc1 = String::from("123").as_crc();
    assert_eq!(expect, crc1);
    let crc2 = Slice::from("123").as_crc();
    assert_eq!(expect, crc2);
    let buf = "123".as_bytes();
    let crc3 = buf.as_crc();
    assert_eq!(expect, crc3);
    let crc4 = Vec::from(buf).as_crc();
    assert_eq!(expect, crc4);

}

#[test]
fn test_mask() {
    let crc = CRC::value("foo".as_bytes());
    assert_ne!(crc, CRC::mask(crc));
    assert_ne!(crc, CRC::mask(CRC::mask(crc)));
    assert_eq!(crc, CRC::unmask(CRC::mask(crc)));
    assert_eq!(crc, CRC::unmask(CRC::unmask(CRC::mask(CRC::mask(crc)))));
}

#[test]
fn test_mask2() {
    let a0 = [1];
    let a1 = [0, 97, 98, 99, 100];
    let a2 = [1, 97, 98, 99, 100];
    let crc0 = a1[1..].as_crc_extend(a0.as_crc()).to_mask();
    let crc1 = a2.as_crc().to_mask();
    println!("crc0: {}, crc1: {}", crc0, crc1);
    assert_eq!(crc0, crc1);
}