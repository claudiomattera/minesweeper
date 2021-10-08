
/// Format and write text to the WASM-4 debug console
///
/// # Panics
///
/// Panic if the resulting string is larger than
/// [`STRING_BUFFER_PTR`](crate::memory::STRING_BUFFER_PTR), or if the
/// resulting string is not valid UTF-8.
///
/// # Examples
///
/// ```no_run
/// let h = 12;
/// let pi = 3.14;
/// debug!("There are {} hours in a day, and pi is {}", h, pi);
/// ```
#[cfg(feature = "debug")]
#[macro_export]
macro_rules! debug {
    ( $format:expr $(, $arg:expr)* ) => {
        {
            use bare_io::{Cursor, Write};

            let mut string_buffer: [u8; 256] = [0; 256];
            let mut cursor = Cursor::new(&mut string_buffer[..]);
            write!(&mut cursor, $format, $($arg,)*)
                .expect("!write");
            let ending = cursor.position() as usize;
            let raw = &string_buffer[..ending];
            let s = unsafe { std::str::from_utf8_unchecked(raw) };
            crate::wasm4::trace(s);
        }
    };
}

#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! debug {
    ( $format:expr $(, $arg:expr)* ) => {}
}
