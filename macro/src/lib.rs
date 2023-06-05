extern crate proc_macro;
extern crate syn;

use std::{cmp::Ordering, str::FromStr};

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use syn::{parse_macro_input, LitStr};

#[proc_macro_hack]
pub fn four_char_code(input: TokenStream) -> TokenStream {
    let input = &parse_macro_input!(input as LitStr);
    let mut bytes = {
        let value = input.value();

        match value.len().cmp(&4) {
            Ordering::Less => {
                return syn::Error::new_spanned(input, "four char code is too short")
                    .into_compile_error()
                    .into();
            }
            Ordering::Greater => {
                return syn::Error::new_spanned(input, "four char code is too long")
                    .into_compile_error()
                    .into();
            }
            _ => (),
        }
        let value = value.as_bytes();
        unsafe {
            [
                *value.get_unchecked(0),
                *value.get_unchecked(1),
                *value.get_unchecked(2),
                *value.get_unchecked(3),
            ]
        }
    };

    {
        let mut null_streak = true;

        let mut i = 3usize;
        loop {
            let mut c = bytes[i];
            if c == 0 {
                if null_streak {
                    c = 0x20;
                    bytes[i] = c;
                } else {
                    return syn::Error::new_spanned(input, "invalid char in four char code")
                        .into_compile_error()
                        .into();
                }
            } else {
                null_streak = false;
            }

            if c <= b'\x1f' || c >= b'\x7f' {
                return syn::Error::new_spanned(input, "invalid char in four char code")
                    .into_compile_error()
                    .into();
            }

            if i == 0 {
                break;
            }
            i -= 1;
        }
    }

    proc_macro::TokenStream::from_str(
        &proc_macro::Literal::u32_suffixed(u32::from_be_bytes(bytes)).to_string(),
    )
    .unwrap()
}
