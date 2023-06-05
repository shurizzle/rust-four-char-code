# four-char-code

![GitHub Workflow Status (with branch)](https://img.shields.io/github/actions/workflow/status/shurizzle/rust-four-char-code/unit-test-v1.yml?branch=v1&style=for-the-badge)

> A FourCC ("four-character code") is a sequence of four bytes (typically ASCII) used to uniquely identify data formats. It originated from the OSType or ResType metadata system used in classic Mac OS and was adopted for the Amiga/Electronic Arts Interchange File Format and derivatives. The idea was later reused to identify compressed data types in QuickTime and DirectShow.

*Widipedia*

### #![no_std]

Enable `#![no_std]` support by disabling the default `std` feature:

```toml
[dependencies]
four-char-code = { version = "1", default-features = false }
```

### MSRV

1.32.0 (in rust <1.38.0 `fcc_format!` is not supported)
