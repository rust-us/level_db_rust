const B: u8 = 128;

///
///
///
pub trait Coding {
    fn varint32(self, buf: &mut [u8]);
}

impl Coding for u32
{
    fn varint32(self, buf: &mut [u8]) {
        // if self < 1 << 7 {
        //     buf[0] = self as u8;
        // } else if self < 1 << 14 {
        //     buf[0] = self | B;
        //     println!("{:b}", buf[0]);
        //     buf[1] = buf[0] >> 7;
        //     println!("{:b}", buf[1]);
        // } else if self < 1 << 21 {
        //     buf[0] = (self | B as u8) as u8;
        //     println!("{:b}", buf[0]);
        //     buf[1] = (buf[0] >> 7) | B;
        //     println!("{:b}", buf[1]);
        //     buf[2] = buf[1] >> 14;
        //     println!("{:b}", buf[2]);
        // } else if self < 1 << 28 {
        //     buf[0] = (self | B as u8) as u8;
        //     println!("{:b}", buf[0]);
        //     buf[1] = (buf[0] >> 7) | B;
        //     println!("{:b}", buf[1]);
        //     buf[2] = (buf[1] >> 14) | B;
        //     println!("{:b}", buf[2]);
        //     buf[3] = buf[2] >> 21;
        // } else {
        //     buf[0] = (self | B as u8) as u8;
        //     println!("{:b}", buf[0]);
        //     buf[1] = (buf[0] >> 7) | B;
        //     println!("{:b}", buf[1]);
        //     buf[2] = (buf[1] >> 14) | B;
        //     println!("{:b}", buf[2]);
        //     buf[3] = (buf[2] >> 21) | B;
        //     println!("{:b}", buf[3]);
        //     buf[4] = buf[3] >> 28;
        // }
    }
}