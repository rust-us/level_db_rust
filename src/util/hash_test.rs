use crate::util::hash::{Hash};

#[test]
fn test_hash() {
    let val = "aabbccd";
    let hash_val = Hash::hash_char(val.as_bytes(), 3);
    println!("hash:{}", hash_val);

    let val = "aabbcc";
    let hash_val = Hash::hash_char(val.as_bytes(), 3);
    println!("hash:{}", hash_val);

    let val = "aabbc";
    let hash_val = Hash::hash_char(val.as_bytes(), 3);
    println!("hash:{}", hash_val);
}

#[test]
fn test_hash_code() {
    let data1: Vec<u8> = vec![0x62];
    let data2: Vec<u8> = vec![0xc3, 0x97];
    let data3: Vec<u8> = vec![0xe2, 0x99, 0xa5];
    let data4: Vec<u8> = vec![0xe1, 0x80, 0xb9, 0x32];
    let data5: Vec<u8> = vec![0x01, 0xc0, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00,
                               0x14, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x04, 0x00,
                               0x00, 0x00, 0x00, 0x14,
                               0x00, 0x00, 0x00, 0x18,
                               0x28, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00,
                               0x02, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00];

    let hash_val = Hash::hash_char(&vec![],0xbc9f1d34);
    assert_eq!(0xbc9f1d34, hash_val);

    let hash_val = Hash::hash_char(&data1, 0xbc9f1d34);
    assert_eq!(0xef1345c4, hash_val);

    let hash_val = Hash::hash_char(&data2, 0xbc9f1d34);
    assert_eq!(0x5b663814, hash_val);
    let hash_val = Hash::hash_char(&data3, 0xbc9f1d34);
    assert_eq!(0x323c078f, hash_val);

    let hash_val = Hash::hash_char(&data4, 0xbc9f1d34);
    assert_eq!(0xed21633a, hash_val);

    let hash_val = Hash::hash_char(&data5, 0x12345678);
    assert_eq!(0xf333dabb, hash_val);
}
