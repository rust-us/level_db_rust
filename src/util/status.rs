/// 使用 Status 这个类来得到你的函数的返回的状态
///
use std::borrow::{Borrow, Cow};
use std::io::Write;
use std::ptr;
use std::ptr::{NonNull, null};
use crate::util::slice::Slice;
use crate::util::status::Code::kOk;

#[derive(Debug)]
pub struct Status {
    /// state_[0, 3] 字节代表消息的长度，这个长度是从 state_[5, ...] 开始的，前面的 5 个字节不算。
    /// state_[4] 字节代表消息的类型，就是上面介绍的 enum Code 的 6 种类型。
    /// state_[5, ...] 代表实际的消息体。

    /// OK status has a null state_.  Otherwise, state_ is a new[] array
    /// of the following form:
    ///    state_[0..3] == length of message
    ///    state_[4]    == code
    ///    state_[5..]  == message
    pub state_: String,
}

impl Default for Status {
    fn default() -> Self {
        unsafe {
            Self {
                state_: String::new()
            }
        }
    }
}

impl Status {
    /// 构造一个 Status
    fn off(state: String) -> Status {
        unsafe {
            Status {
                state_: state
            }
        }
    }

    /// 构造一个 Status
    pub fn of(code: Code, msg: Slice, msg2:Slice) -> Status {
        let len2: usize = msg2.len();
        let msgSize: usize = msg.len() + (2 + len2);

        let mut vecStatus: Vec<u8> = Vec::with_capacity(msgSize);

        let msgSizeV: u32 = msgSize.try_into().unwrap();

        // 0,1,2,3
        // unsafe {
        //     vecStatus.as_mut_ptr().cast::<u32>().offset(0)
        //         .write(msgSizeV);
        // }
        vecStatus.push(msgSizeV as u8);
        vecStatus.push((msgSizeV >> 8) as u8);
        vecStatus.push((msgSizeV >> 16) as u8);
        vecStatus.push((msgSizeV >> 24) as u8);

        // 4
        vecStatus.push(code as u8);

        // data 1
        vecStatus.write(msg.as_ref());

        if len2 > 0 {
            vecStatus.write(":".as_bytes());
            vecStatus.write(" ".as_bytes());
            vecStatus.write(msg2.as_ref());
        }

        unsafe {
            return Status::off(String::from_utf8_unchecked(vecStatus));
        }
    }

//     fn code(&self) -> Code {
//         if self.state_ == nullptr
//             return Code::kOk;
//
//
//         return () ?  : self.state_[4];
//     }
//
//     /// Returns true iff the status indicates success.
//     pub fn ok(&self) -> bool {
//         self.state_ == NonNull
//         // let code: Cow<String> = Slice::borrow_data(&Slice::from(String::from("123").into()));
//         // true
//     }
//
//     /// Returns true iff the status indicates a NotFound error.
//     pub fn is_not_found(&self) -> bool {
//         true
//     }
//
//     /// Returns true iff the status indicates a Corruption error.
//     pub fn is_corruption(&self) -> bool {
//         true
//     }
//
//     /// Returns true iff the status indicates an IOError.
//     pub fn is_io_error(&self) -> bool {
//         true
//     }
//
//     /// Returns true iff the status indicates a NotSupportedError.
//     pub fn is_not_supported_error(&self) -> bool {
//         true
//     }
//
//     /// Returns true iff the status indicates an InvalidArgument.
//     pub fn is_invalid_argument(&self) -> bool {
//         true
//     }
//
//     /// Return a string representation of this status suitable for printing.
//     /// Returns the string "OK" for success.
//     pub fn to_string(&self) -> String {
//         format!("{}", "")
//     }
}

impl<'a> Status {
    /// 返回 ok的 Status
    pub fn ok(&self) -> Status {
        Status::default()
    }

    // /// 返回 ok的 Status
    // pub fn not_found(msg: Slice) -> Status {
    //     Status::default()
    // }
    //
    // /// 借取 Slice 中的数据, 调用方只拥有读权限
    // pub fn copy_state(&self) -> Cow<'a, String> {
    //     let size : u32;
    //     let str = unsafe {
    //         String::from_raw_parts(self.data.as_ptr(), self.len, self.len)
    //     };
    //     Cow::Owned(str)
    // }
}

/// Status 的状态
#[derive(Debug, PartialEq)]
pub enum Code {
    kOk = 0,
    kNotFound = 1,
    kCorruption = 2,
    kNotSupported = 3,
    kInvalidArgument = 4,
    kIOError = 5,
}
