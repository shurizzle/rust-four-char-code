extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[inline]
fn str_to_u32(value: &str) -> u32 {
    if value.len() != 4 {
        panic!("{} is not a valid four char code", value);
    }

    let mut res: u32 = 0;
    unsafe {
        std::ptr::copy(
            value.as_ptr(),
            &mut res as *mut _ as *mut u8,
            std::mem::size_of::<u8>() * 4,
        );
    }
    u32::from_be(res)
}

#[proc_macro_hack]
pub fn four_char_code(input: TokenStream) -> TokenStream {
    let value = str_to_u32(&parse_macro_input!(input as LitStr).value());

    let expanded = quote! {
        ::four_char_code::FourCharCode(#value)
    };

    TokenStream::from(expanded)
}
