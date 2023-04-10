use crate::util::Arena;

#[test]
fn test_memory_usage() {
    let mut arena = Arena::default();
    let _buf0 = arena.allocate(12);
    let _buf1 = arena.allocate(16);
    assert_eq!(4096, arena.memory_usage());
    let _buf2 = arena.allocate(3900);
    assert_eq!(4096, arena.memory_usage());
    let _buf3 = arena.allocate(1200);
    assert_eq!(4096 + 1200, arena.memory_usage());
}

#[test]
fn test_allocate() {
    let mut arena = Arena::default();
    for i in 0..=12 {
        let byte_size = 1 << i;
        let buf = arena.allocate(byte_size);
        assert_eq!(byte_size, buf.len());
    }
    assert_eq!(8192, arena.memory_usage());
}

#[test]
fn test_allocate_align() {
    let mut arena = Arena::default();
    for i in 0..=12 {
        let byte_size = 1 << i;
        let buf = arena.allocate_align(byte_size, 8);
        assert_eq!(byte_size, buf.len());
    }
    assert_eq!(4096 * 2, arena.memory_usage());
}