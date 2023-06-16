//
// #![allow(non_snake_case)]
//
// use std::backtrace::Backtrace;
// use std::sync::Arc;
//
// use crate::util::error::ErrorCodeBacktrace;
// use crate::util::error::ErrorCode;
//
// macro_rules! build_exceptions {
//     ($($(#[$meta:meta])* $body:ident($code:expr)),*$(,)*) => {
//         impl ErrorCode {
//             $(
//
//                 paste::item! {
//                     $(
//                         #[$meta]
//                     )*
//                     pub const [< $body:snake:upper >]: u16 = $code;
//                 }
//                 $(
//                     #[$meta]
//                 )*
//                 pub fn $body(display_text: impl Into<String>) -> ErrorCode {
//                     let bt = Some(ErrorCodeBacktrace::Origin(Arc::new(Backtrace::capture())));
//                     ErrorCode::create(
//                         $code,
//                         display_text.into(),
//                         None,
//                         bt,
//                     )
//                 }
//             )*
//         }
//     }
// }
//
// build_exceptions! {
//     Ok(0),
//     Internal(1001),
// }
