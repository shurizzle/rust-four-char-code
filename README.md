# four-char-code

![GitHub Workflow Status (with branch)](https://img.shields.io/github/actions/workflow/status/shurizzle/rust-four-char-code/unit-test.yml?branch=master&style=for-the-badge)
[![Crates.io](https://img.shields.io/crates/v/four-char-code?style=for-the-badge)](https://crates.io/crates/four-char-code)
[![docs.rs](https://img.shields.io/docsrs/four-char-code?style=for-the-badge)](https://docs.rs/four-char-code)
![Crates.io](https://img.shields.io/crates/l/four-char-code?style=for-the-badge)

> A FourCC ("four-character code") is a sequence of four bytes (typically ASCII) used to uniquely identify data formats. It originated from the OSType or ResType metadata system used in classic Mac OS and was adopted for the Amiga/Electronic Arts Interchange File Format and derivatives. The idea was later reused to identify compressed data types in QuickTime and DirectShow.

*Widipedia*

### #![no_std]

Enable `#![no_std]` support by disabling the default `std` feature:

```toml
[dependencies]
four-char-code = { version = "2", default-features = false }
```

### MSRV

1.57.0

For older rust versions take a look at version 1.
