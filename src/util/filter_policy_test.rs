



#[test]
fn test_new() {
    let _bloom_filter = BloomFilterPolicy::new(8);
    println!("hash:{}", "a");
    // assert_eq!(bloom_filter, null());

    let _bloom_filter = BloomFilterPolicy::new(800);
    println!("hash:{}", "a");

}