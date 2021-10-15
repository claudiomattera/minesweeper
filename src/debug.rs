// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Debug macros

/// Format and write text to the WASM-4 debug console
///
/// # Panics
///
/// Panic if the resulting string is larger than 256 bytes, or if the
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
            let s = unsafe { core::str::from_utf8_unchecked(raw) };
            crate::wasm4::trace(s);
        }
    };
}

/// Pretend to format and write text to the WASM-4 debug console
///
/// This is the definition of `debug!` macro in case debugging is disabled.
#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! debug {
    ( $format:expr $(, $arg:expr)* ) => {
        {
            $(
                let _ = $arg;
            )*
        }
    }
}
