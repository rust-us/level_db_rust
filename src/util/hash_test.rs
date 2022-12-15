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