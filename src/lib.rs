#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_camel_case_types)]

extern crate four_char_code_macros_impl;
extern crate proc_macro_hack;

use core::{cmp::Ordering, fmt};

#[cfg(feature = "std")]
use std::string::{String, ToString};

/// An enum representing a conversion (eg. string->fcc or format->fcc) error
#[derive(Debug, Clone, Copy)]
pub enum FccConversionError {
    /// Given string is > 4 bytes
    TooLong,
    /// Given string is < 4 bytes
    TooShort,
    /// Given string contains a non printable ascii char
    InvalidChar,
}

impl FccConversionError {
    pub fn description(&self) -> &str {
        match self {
            Self::TooLong => "four char code is too long",
            Self::TooShort => "four char code is too short",
            Self::InvalidChar => "invalid char in four char code",
        }
    }
}

impl fmt::Display for FccConversionError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(FccConversionError::description(self), f)
    }
}

#[cfg(feature = "std")]
impl ::std::error::Error for FccConversionError {
    #[inline]
    fn description(&self) -> &str {
        FccConversionError::description(self)
    }
}

type Result<T> = core::result::Result<T, FccConversionError>;

/// The main structure, actually a u32.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct FourCharCode(u32);

fn from_bytes(mut bytes: [u8; 4]) -> Result<FourCharCode> {
    let mut null_streak = true;

    let mut i = 3usize;
    loop {
        let mut c = bytes[i];
        if c == 0 {
            if null_streak {
                c = 0x20;
                bytes[i] = c;
            } else {
                return Err(FccConversionError::InvalidChar);
            }
        } else {
            null_streak = false;
        }

        if c <= b'\x1f' || c >= b'\x7f' {
            return Err(FccConversionError::InvalidChar);
        }

        if i == 0 {
            break;
        }
        i -= 1;
    }

    Ok(FourCharCode(u32::from_be_bytes(bytes)))
}

impl FourCharCode {
    /// Returns a [FourCharCode] if value is valid, an error describing the problem otherwise.
    #[inline]
    pub fn new(value: u32) -> Result<Self> {
        from_bytes(u32::to_be_bytes(value))
    }

    /// Returns a [FourCharCode] containing the given value.
    /// # Safety
    /// Passing an invalid value can cause a panic
    #[inline]
    pub const unsafe fn new_unchecked(value: u32) -> Self {
        Self(value)
    }

    /// Returns a [FourCharCode] if values are valid, an error describing the problem otherwise.
    #[inline]
    pub fn from_array(value: [u8; 4]) -> Result<Self> {
        from_bytes(value)
    }

    /// Returns a [FourCharCode] if slice is valid, an error describing the problem otherwise.
    pub fn from_slice(value: &[u8]) -> Result<Self> {
        match value.len().cmp(&4) {
            Ordering::Less => return Err(FccConversionError::TooShort),
            Ordering::Greater => return Err(FccConversionError::TooLong),
            _ => (),
        }

        from_bytes(unsafe {
            [
                *value.get_unchecked(0),
                *value.get_unchecked(1),
                *value.get_unchecked(2),
                *value.get_unchecked(3),
            ]
        })
    }

    /// Returns a [FourCharCode] if string is valid, an error describing the problem otherwise.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(value: &str) -> Result<Self> {
        Self::from_slice(value.as_bytes())
    }
}

impl PartialEq<u32> for FourCharCode {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<u32> for FourCharCode {
    #[inline]
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialEq<str> for FourCharCode {
    fn eq(&self, other: &str) -> bool {
        if let Ok(other) = Self::from_str(other) {
            *self == other
        } else {
            false
        }
    }
}

impl PartialEq<&str> for FourCharCode {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.eq(*other)
    }
}

impl PartialOrd<str> for FourCharCode {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        if let Ok(other) = Self::from_str(other) {
            self.partial_cmp(&other)
        } else {
            None
        }
    }
}

impl PartialOrd<&str> for FourCharCode {
    #[inline]
    fn partial_cmp(&self, other: &&str) -> Option<Ordering> {
        self.partial_cmp(*other)
    }
}

impl PartialEq<[u8]> for FourCharCode {
    fn eq(&self, other: &[u8]) -> bool {
        if let Ok(other) = Self::from_slice(other) {
            *self == other
        } else {
            false
        }
    }
}

impl PartialEq<&[u8]> for FourCharCode {
    #[inline]
    fn eq(&self, other: &&[u8]) -> bool {
        self.eq(*other)
    }
}

impl PartialOrd<[u8]> for FourCharCode {
    fn partial_cmp(&self, other: &[u8]) -> Option<Ordering> {
        if let Ok(other) = Self::from_slice(other) {
            self.partial_cmp(&other)
        } else {
            None
        }
    }
}

impl PartialOrd<&[u8]> for FourCharCode {
    #[inline]
    fn partial_cmp(&self, other: &&[u8]) -> Option<Ordering> {
        self.partial_cmp(*other)
    }
}

impl PartialEq<[u8; 4]> for FourCharCode {
    fn eq(&self, other: &[u8; 4]) -> bool {
        if let Ok(other) = Self::from_array(*other) {
            *self == other
        } else {
            false
        }
    }
}

impl PartialOrd<[u8; 4]> for FourCharCode {
    fn partial_cmp(&self, other: &[u8; 4]) -> Option<Ordering> {
        if let Ok(other) = Self::from_array(*other) {
            self.partial_cmp(&other)
        } else {
            None
        }
    }
}

impl fmt::Debug for FourCharCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let be = self.0.to_be_bytes();
        f.debug_tuple("FourCharCode")
            .field(&unsafe { core::str::from_utf8_unchecked(&be[..]) })
            .finish()
    }
}

#[cfg(feature = "std")]
impl ToString for FourCharCode {
    #[inline]
    fn to_string(&self) -> String {
        let bytes = self.0.to_be_bytes();
        unsafe { core::str::from_utf8_unchecked(&bytes[..]) }.to_string()
    }
}

impl From<FourCharCode> for u32 {
    #[inline]
    fn from(value: FourCharCode) -> Self {
        value.0
    }
}

#[cfg(feature = "std")]
impl From<FourCharCode> for String {
    #[inline]
    fn from(value: FourCharCode) -> Self {
        value.to_string()
    }
}

#[doc(hidden)]
#[cfg(ge_1_38_0)]
pub mod __private {
    use core::fmt::Write;

    use super::{FccConversionError, FourCharCode};

    struct FccBuf {
        buf: [u8; 4],
        len: usize,
        err: Option<FccConversionError>,
    }

    impl FccBuf {
        #[inline(always)]
        fn new() -> Self {
            Self {
                buf: [0; 4],
                len: 0,
                err: None,
            }
        }
    }

    impl core::fmt::Write for FccBuf {
        fn write_char(&mut self, c: char) -> core::fmt::Result {
            if !c.is_ascii() || c.is_control() {
                self.err = Some(FccConversionError::InvalidChar);
                Err(core::fmt::Error)
            } else if self.len == 4 {
                self.err = Some(FccConversionError::TooLong);
                Err(core::fmt::Error)
            } else {
                unsafe { *self.buf.get_unchecked_mut(self.len) = c as u8 };
                self.len += 1;
                Ok(())
            }
        }

        #[inline]
        fn write_fmt(mut self: &mut Self, args: core::fmt::Arguments<'_>) -> core::fmt::Result {
            core::fmt::write(&mut self, args)
        }

        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            for c in s.chars() {
                self.write_char(c)?;
            }
            Ok(())
        }
    }

    pub fn fcc_format(
        args: core::fmt::Arguments<'_>,
    ) -> core::result::Result<FourCharCode, FccConversionError> {
        let mut buf = FccBuf::new();
        buf.write_fmt(args).map_err(|_| buf.err.take().unwrap())?;
        if buf.len != 4 {
            return Err(FccConversionError::TooShort);
        }
        Ok(FourCharCode(u32::from_be_bytes(buf.buf)))
    }
}

#[derive(proc_macro_hack::ProcMacroHack)]
enum _26four_char_code_macros_impl_14four_char_code {
    Value = (
        stringify! {
            #[doc(hidden)]
            pub use four_char_code_macros_impl::_proc_macro_hack_four_char_code;

            /// Create a checked [FourCharCode] at compile time
            #[macro_export]
            macro_rules! four_char_code {
                ($($proc_macro:tt)*) => {
                    {
                        #[derive($crate::_proc_macro_hack_four_char_code)]
                        #[allow(dead_code)]
                        enum ProcMacroHack {
                            Value = (stringify!($($proc_macro)*), 0).1
                        }
                        unsafe { $crate::FourCharCode::new_unchecked(proc_macro_call!()) }
                    }
                };
            }
        },
        0,
    )
        .1,
}

#[cfg(ge_1_38_0)]
/// Returns a [FourCharCode] from a `format!` like expression without allocation if valid.
/// Returns an error describing the problem otherwise.
#[macro_export]
macro_rules! fcc_format {
    ($fmt:expr) => {
        $crate::__private::fcc_format(::core::format_args!($fmt))
    };
    ($fmt:expr, $($args:tt)*) => {
        $crate::__private::fcc_format(::core::format_args!($fmt, $($args)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEX: FourCharCode = four_char_code!("hex_");

    #[test]
    fn invalid() {
        assert!(FourCharCode::new(1).is_err());
        assert!(FourCharCode::from_str("").is_err());
        assert!(FourCharCode::from_str("test1").is_err());
        assert!(FourCharCode::from_str("\x7f___").is_err());
    }

    #[test]
    fn valid() {
        assert_eq!(HEX, "hex_");
        let ui32 = FourCharCode::from_str("ui32");
        assert!(ui32.is_ok());
        assert_eq!(ui32.unwrap(), "ui32");
    }

    #[cfg(ge_1_38_0)]
    #[test]
    fn format() {
        let f1mn = fcc_format!("F{}Mn", 1);
        assert!(f1mn.is_ok());
        assert_eq!(f1mn.unwrap(), "F1Mn");
    }
}
