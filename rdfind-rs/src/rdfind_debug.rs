// Ported from orig_src/RdfindDebug.hh
// Provides a conditional debug macro similar to the C++ RDDEBUG macro.
// Set the RDFIND_DEBUG environment variable to enable debug output.

#[macro_export]
macro_rules! rddebug {
    ($($arg:tt)*) => {
        if std::env::var("RDFIND_DEBUG").is_ok() {
            eprintln!("{}:{}: {}", file!(), line!(), format!($($arg)*));
        }
    };
}
