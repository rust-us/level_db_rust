mod test {
    use crate::traits::coding_trait::{Coding32, Coding64, CodingTrait};
    use crate::util::slice::Slice;
    use crate::util::coding::{Coding};

    #[test]
    fn test_put_fixed32() {
        let mut string = String::from("encode:");
        let value = 65535;
        Coding::put_fixed32(&mut string, value);
        let mut assert_stirng = String::from("encode:");
        let mut encode_buf: [u8; 4] = [0, 0, 0, 0];
        Coding::encode_fixed32(value, &mut encode_buf, 0);
        for b in encode_buf {
            assert_stirng.push(char::from(b));
        }
        assert_eq!(assert_stirng, string);
    }

    #[test]
    fn test_put_fixed64() {
        let mut string = String::from("encode:");
        let value = 65535;
        Coding::put_fixed64(&mut string, value);
        let mut assert_stirng = String::from("encode:");
        let mut encode_buf: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        Coding::encode_fixed64(value, &mut encode_buf, 0);
        for b in encode_buf {
            assert_stirng.push(char::from(b));
        }
        assert_eq!(assert_stirng, string);
    }

    #[test]
    fn test_encode_varint32() {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        let value: u32 = 65534;
        let offset = Coding::encode_varint32(value, &mut buf, 0);
        assert_eq!(offset, 2);
    }

    #[test]
    fn test_encode_varint64() {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        let value: u64 = 65534;
        let offset = Coding::encode_varint64(value, &mut buf, 0);
        assert_eq!(offset, 2);
    }

    #[test]
    fn test_encode_fixed32() {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        let value: u32 = 65534;
        let offset = Coding::encode_fixed32(value, &mut buf, 0);
        assert_eq!(offset, 4);
    }

    #[test]
    fn test_encode_fixed64() {
        let mut buf: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let value: u64 = 65534;
        let offset = Coding::encode_fixed64(value, &mut buf, 0);
        assert_eq!(offset, 8);
    }

    #[test]
    fn test_varint_u32() {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        let value: u32 = 65534;
        println!("{:b}", value);
        let offset = value.varint(&mut buf, 0);
        println!("{:?}", buf);
        println!("{:?}", offset);
        println!("{:b}", buf[0]);
    }

    #[test]
    fn test_varint_u64() {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        let value: u64 = 65534;
        println!("{:b}", value);
        let offset = value.varint(&mut buf, 0);
        println!("{:?}", buf);
        println!("{:?}", offset);
        println!("{:b}", buf[0]);
    }

    #[test]
    fn test_fixed_u32() {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        let value: u32 = 123;
        println!("{:b}", value);
        let offset = value.fixedint(&mut buf, 0);
        println!("{:?}", buf);
        println!("{:?}", offset);
        println!("{:b}", buf[0]);
    }

    #[test]
    fn test_fixed_u64() {
        let mut buf: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let value: u64 = 123;
        println!("{:b}", value);
        let offset = value.fixedint(&mut buf, 0);
        println!("{:?}", buf);
        println!("{:?}", offset);
        println!("{:b}", buf[0]);
    }

    #[test]
    fn test_varint_length() {
        let len = Coding::varint_length(65535);
        println!("{:?}", len);
    }

    #[test]
    fn test_put_length_prefixed_slice() {
        let data = 12_u32;
        let mut string = String::from("len:");
        let mut slice = Slice::from_buf(data.to_le_bytes().as_mut_slice());
        Coding::put_length_prefixed_slice(&mut string, &mut slice);
        println!("{:?}", string)
    }

    #[test]
    fn test_get_length_prefixed_slice() {
        let data = 12_u32;
        let mut string = String::from("len:");
        let mut slice = Slice::from_buf(data.to_le_bytes().as_mut_slice());
        Coding::put_length_prefixed_slice(&mut string, &mut slice);
    }
}
