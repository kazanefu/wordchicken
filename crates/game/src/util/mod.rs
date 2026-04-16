pub mod words;

#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            println!($($arg)*);
        }
    };
}