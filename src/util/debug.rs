use std::fmt::format;
use std::io::Write;
#[macro_export]
macro_rules! debug {
    () => {
        if (crate::util::r#const::DEBUG_ENABLE) {
            std::io::stdout().write("\n".as_bytes()).unwrap();
        }
    };
    ($($arg:tt)*) => {{
        use std::io::Write;
        if(crate::util::r#const::DEBUG_ENABLE) {
            std::io::stdout().write(format!($($arg)*).as_bytes());
            debug!();
        }
    }};
}
