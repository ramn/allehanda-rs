#[macro_export]
macro_rules! eprintln {
    ($fmt:expr) => {
        use std::io::Write;
        writeln!(std::io::stderr(), $fmt).unwrap();
    };
    ($fmt:expr, $($arg:tt)*) => {
        use std::io::Write;
        writeln!(std::io::stderr(), $fmt, $( $arg )*).unwrap();
    };
}
