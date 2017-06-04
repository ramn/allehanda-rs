#[macro_export]
macro_rules! eprintln {
    ($fmt:expr) => {
        writeln!(std::io::stderr(), $fmt).unwrap();
    };
    ($fmt:expr, $($arg:tt)*) => {
        writeln!(std::io::stderr(), $fmt, $( $arg )*).unwrap();
    };
}
