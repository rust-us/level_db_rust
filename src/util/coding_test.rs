mod test {
    use crate::traits::coding_trait::{Coding32, Coding64, CodingTrait};
    use crate::util::coding::{Coding};

    #[test]
    fn test_put_fixed32() {
        let mut dst = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let value = 65535;
        Coding::put_fixed32(&mut dst, 2, value);
        assert_eq!([0, 0, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0] as [u8; 12], dst);
    }

    #[test]
    fn test_put_fixed64() {
        let mut dst = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let value = 65535;
        Coding::put_fixed64(&mut dst, 2, value);
        assert_eq!([0, 0, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0] as [u8; 12], dst);
    }

    #[test]
    fn test_put_varint32() {
        let mut value = 65535;
        let mut dst = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let offset = Coding::put_varint32(&mut dst, 2, value);
        println!("offset:{:?}", offset);
        assert_eq!(offset, 4);
        println!("dst:{:?}", dst);
        assert_eq!([0, 0, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0] as [u8; 12], dst);
    }

    #[test]
    fn test_put_varint64() {
        let mut value = 65535;
        let mut dst = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let offset = Coding::put_varint64(&mut dst, 2, value);
        println!("offset:{:?}", offset);
        assert_eq!(offset, 4);
        println!("dst:{:?}", dst);
        assert_eq!([0, 0, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0] as [u8; 12], dst);
    }

    #[test]
    fn test_encode_varint32() {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        let mut value: u32 = 65534;
        let offset = Coding::encode_varint32(value, &mut buf, 0);
        println!("offset:{:?}", offset);
        assert_eq!(offset, 2);
        println!("buf:{:?}", buf);
        assert_eq!(buf, [254, 255, 3, 0]);
    }

    #[test]
    fn test_encode_varint64() {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        let mut value: u64 = 65535;
        let offset = Coding::encode_varint64(value, &mut buf, 0);
        println!("offset:{:?}", offset);
        assert_eq!(offset, 2);
        println!("buf:{:?}", buf);
        assert_eq!(buf, [255, 255, 3, 0]);
    }

    #[test]
    fn test_encode_fixed32() {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        let mut value: u32 = 65534;
        let offset = Coding::encode_fixed32(value, &mut buf, 0);
        assert_eq!(offset, 4);
        println!("offset:{:?}", offset);
        assert_eq!(buf, [254, 255, 0, 0]);
        println!("buf:{:?}", buf);
    }

    #[test]
    fn test_encode_fixed64() {
        let mut buf: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let mut value: u64 = 65535;
        let offset = Coding::encode_fixed64(value, &mut buf, 0);
        assert_eq!(offset, 8);
        println!("offset:{:?}", offset);
        assert_eq!(buf, [255, 255, 0, 0, 0, 0, 0, 0]);
        println!("buf:{:?}", buf);
    }

    #[test]
    fn test_varint_u32() {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        let value: u32 = 65534;
        println!("value[binary]:{:b}", value);
        let offset = value.varint(&mut buf, 0);
        println!("offset:{:?}", offset);
        println!("buf:{:?}", buf);
        assert_eq!(buf, [255, 255, 3, 0]);
    }

    #[test]
    fn test_varint_u64() {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        let value: u64 = 65534;
        println!("value[binary]:{:b}", value);
        let offset = value.varint(&mut buf, 0);
        println!("offset:{:?}", offset);
        println!("buf:{:?}", buf);
    }

    #[test]
    fn test_fixed_u32() {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        let value: u32 = 123;
        println!("value[binary]:{:b}", value);
        let offset = value.fixedint(&mut buf, 0);
        println!("offset:{:?}", offset);
        println!("buf:{:?}", buf);
    }

    #[test]
    fn test_fixed_u64() {
        let mut buf: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let value: u64 = 123;
        println!("value[binary]:{:b}", value);
        let offset = value.fixedint(&mut buf, 0);
        println!("offset:{:?}", offset);
        println!("buf:{:?}", buf);
    }

    #[test]
    fn test_varint_length() {
        let len = Coding::varint_length( 65535 as u64);
        println!("len: {:?}", len);
        assert_eq!(len, 3);
    }

    #[test]
    fn test_put_length_prefixed_slice() {
        // let mut buf: [u8; 4] = [0, 0, 0, 0];
        // Coding::encode_fixed32(&data, &mut buf, 0);
        // let mut string = String::from("len:");
        // let slice = Slice::from("data");
        // let mut slice = Slice::from_buf(data.to_le_bytes().as_mut_slice());
        // Coding::put_length_prefixed_slice(&mut string, &mut slice);
        // println!("{:?}", string)
    }

    #[test]
    fn test_get_length_prefixed_slice() {
        // let data = 12_u32;
        // Coding::put_fixed32(, data);
        // let mut string = String::from("len:");
        // let mut slice = Slice::from_buf(data.to_le_bytes().as_mut_slice());
        // Coding::put_length_prefixed_slice(&mut string, &mut slice);
    }

    #[test]
    fn test_decode_fixed32() {
        let mut value = 65535_u32;
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        Coding::encode_fixed32(value, &mut buf, 0);
        let decode = Coding::decode_fixed32(&mut buf);
        println!("value:{:?}", value);
        assert_eq!(decode, value);
    }

    #[test]
    fn test_decode_fixed64() {
        let mut value = 65535_u64;
        let mut buf: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        Coding::encode_fixed64(value, &mut buf, 0);
        let decode = Coding::decode_fixed64(&mut buf);
        println!("value:{:?}", value);
        assert_eq!(decode, value);
    }
}
