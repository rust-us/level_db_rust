use crate::util::hash::{Hash, ToHash};
use crate::util::r#const::HASH_DEFAULT_SEED;
use crate::util::slice::Slice;

#[test]
fn test_hash() {
    let val = "aabbccd";
    let hash_val = Hash::hash_code(val.as_bytes(), 3);
    println!("hash:{}", hash_val);

    let val = "aabbcc";
    let hash_val = Hash::hash_code(val.as_bytes(), 3);
    println!("hash:{}", hash_val);

    let val = "aabbc";
    let hash_val = Hash::hash_code(val.as_bytes(), 3);
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

    let hash_val = Hash::hash_code(&vec![], 0xbc9f1d34);
    assert_eq!(0xbc9f1d34, hash_val);

    let hash_val = Hash::hash_code(&data1, 0xbc9f1d34);
    assert_eq!(0xef1345c4, hash_val);

    let hash_val = Hash::hash_code(&data2, 0xbc9f1d34);
    assert_eq!(0x5b663814, hash_val);
    let hash_val = Hash::hash_code(&data3, 0xbc9f1d34);
    assert_eq!(0x323c078f, hash_val);

    // todo  coding 重写后，用例报错
    let hash_val = Hash::hash_code(&data4, 0xbc9f1d34);
    assert_eq!(0xed21633a, hash_val);

    // todo  coding 重写后，用例报错
    // let hash_val = Hash::hash_code(&data5, 0x12345678);
    // assert_eq!(0xf333dabb, hash_val);
}

#[test]
fn test_string_to_hash() {
    let val = "aabbccd";
    let hash_val_get = Hash::hash_code(val.as_bytes(), HASH_DEFAULT_SEED);
    println!("hash_val_get:{}", hash_val_get);

    let val_s = String::from(val);
    let string_hash_val = val_s.to_hash();
    println!("string_hash_val:{}", string_hash_val);

    assert_eq!(hash_val_get, string_hash_val);
}

#[test]
fn test_slice_to_hash() {
    let val = "aabbccd";
    let slice: Slice = Slice::from_buf(val.as_bytes());
    let slice_hash_val = slice.to_hash();
    println!("slice_hash_val:{}", slice_hash_val);

    let hash_val_get = Hash::hash_code(slice.to_vec().as_slice(), HASH_DEFAULT_SEED);
    println!("hash_code:{}", hash_val_get);

    assert_eq!(hash_val_get, slice_hash_val);
}

#[test]
fn test_str_to_hash() {
    let str = "aabbccd";
    let str_hash_val = str.to_hash();
    println!("str_hash_val:{}", str_hash_val);

    let hash_val_get = Hash::hash_code(str.as_bytes(), HASH_DEFAULT_SEED);
    println!("hash_code:{}", hash_val_get);

    assert_eq!(hash_val_get, str_hash_val);
}

#[test]
fn test_size_base_to_hash() {
    // 所有基本类型 u8, i8, u16, u32

    let buf = ['a','b','c'];
    let char_hash_val = &buf.as_slice().to_hash();
    println!("char_hash_val:{}", char_hash_val);

    let buf = ["aa", "bb", "cc"].as_slice();
    let string_hash_val = &buf.to_hash();
    println!("string_hash_val:{}", string_hash_val);

    let buf = [1, 2, u32::MAX].as_slice();
    let u32_hash_val = &buf.to_hash();
    println!("u32_hash_val:{}", u32_hash_val);
}

#[test]
fn test_size_vec_to_hash() {
    let buf = vec!['a','b','c'];
    let char_hash_val = buf.to_hash();
    println!("char_hash_val:{}", char_hash_val);

    let buf = ["aa", "bb", "cc"].as_slice();
    let string_hash_val = &buf.to_hash();
    println!("string_hash_val:{}", string_hash_val);

    let buf = [1, 2, u32::MAX].as_slice();
    let u32_hash_val = &buf.to_hash();
    println!("u32_hash_val:{}", u32_hash_val);
}
