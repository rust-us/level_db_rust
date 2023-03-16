


// #[cfg(feature = "debug-macro")]
#[cfg(CORE_DEBUG = "true")]
#[macro_export]
macro_rules! debug {
    () => {
        std::io::stdout().write("\n".as_bytes()).unwrap();
    };
    ($($arg:tt)*) => {{
        use std::io::Write;
        std::io::stdout().write(format!($($arg)*).as_bytes());
        debug!();
    }};
}

// #[cfg(not(feature = "debug-macro"))]
#[cfg(not(CORE_DEBUG = "true"))]
#[macro_export]
macro_rules! debug {
    () => {
    };
    ($($arg:tt)*) => {{
    }};
}