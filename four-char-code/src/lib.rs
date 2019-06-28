extern crate four_char_code_macros_impl;

use proc_macro_hack::proc_macro_hack;

use std::{borrow::Borrow, fmt, slice, str};

#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct FourCharCode(pub u32);

impl FourCharCode {
    #[inline]
    pub fn new(value: u32) -> FourCharCode {
        FourCharCode(value)
    }

    #[inline]
    pub fn to_u32(&self) -> u32 {
        self.0
    }

    pub fn to_string(&self) -> String {
        let value = self.0.to_be();
        let data = unsafe { slice::from_raw_parts(&value as *const _ as *const u8, 4) };
        unsafe { str::from_utf8_unchecked(data) }.to_string()
    }
}

impl fmt::Debug for FourCharCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FourCharCode({:?})", self.to_string())
    }
}

impl Clone for FourCharCode {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl From<u32> for FourCharCode {
    fn from(number: u32) -> FourCharCode {
        FourCharCode(number)
    }
}

impl From<String> for FourCharCode {
    fn from(value: String) -> FourCharCode {
        FourCharCode::from(value.borrow())
    }
}

impl From<&str> for FourCharCode where {
    fn from(value: &str) -> FourCharCode {
        if value.len() != 4 {
            panic!("{} is not a valid four char code", value);
        }

        let mut res: u32 = 0;
        unsafe {
            std::ptr::copy(
                value.borrow().as_ptr(),
                &mut res as *mut _ as *mut u8,
                std::mem::size_of::<u8>() * 4,
            );
        }
        FourCharCode(u32::from_be(res))
    }
}

impl From<FourCharCode> for u32 {
    fn from(fcc: FourCharCode) -> u32 {
        fcc.to_u32()
    }
}

impl From<FourCharCode> for String {
    fn from(fcc: FourCharCode) -> String {
        fcc.to_string()
    }
}

#[proc_macro_hack]
pub use four_char_code_macros_impl::four_char_code;
