use crate::util::hash::{Hash};

#[test]
fn test_hash() {
    let val = "aabbccd";
    let hash_val = Hash::hash(String::from(val), val.len(), 3);
    println!("hash:{}", hash_val);

    let val = "aabbcc";
    let hash_val = Hash::hash(String::from(val), val.len(), 3);
    println!("hash:{}", hash_val);

    let val = "aabbc";
    let hash_val = Hash::hash(String::from(val), val.len(), 3);
    println!("hash:{}", hash_val);
}

#[test]
fn test_hash_code() {
    let data4: Vec<u8> = vec![0xe1, 0x80, 0xb9, 0x32];

    let hash_val = Hash::hash_char(&data4, data4.len(), 3);
    println!("hash:{}", hash_val);
    // 3978388282
    // assert_eq!(0xed21633a, hash_val);
}