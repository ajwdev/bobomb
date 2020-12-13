#[macro_export]
macro_rules! dbg_hex {
    ($i:expr) => {
        println!("[{}:{}] {} = {:#X}", file!(), line!(), stringify!($i), $i)
    };
    ($i:expr,) => { dbg_hex!($i) };
    ($($i:expr),+ $(,)?) => {
        ($(dbg_hex!($i)),+,)
    };
}
