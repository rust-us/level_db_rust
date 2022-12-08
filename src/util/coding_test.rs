mod test {
    use crate::util::coding::Coding;

    #[test]
    fn test_i32() {
        let mut buf: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let x = 127.varint32(&mut buf);
        println!("{:?}", buf);
        println!("{:?}", x);
        println!("{:b}", buf[0]);
    }

    #[test]
    fn test_offset() {
        let buf: [u8; 3] = [0, 1, 2];
        let mut offset: usize = 0;
        offset += 1;
        println!("{:?}", buf[offset])
    }
}
