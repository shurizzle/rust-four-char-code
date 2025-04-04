use std::{ffi::OsString, process::Command};

pub struct Version {
    pub minor: u16,
    pub patch: u16,
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(ge_1_38_0)");

    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| OsString::from("rustc"));
    let mut is_clippy_driver = false;
    let version = loop {
        let mut command = Command::new(&rustc);
        if is_clippy_driver {
            command.arg("--rustc");
        }
        command.arg("--version");

        let output = match command.output() {
            Ok(output) => output,
            Err(e) => {
                let rustc = rustc.to_string_lossy();
                eprintln!("Error: failed to run `{} --version`: {}", rustc, e);
                std::process::exit(1);
            }
        };

        let string = match String::from_utf8(output.stdout) {
            Ok(s) => s,
            Err(e) => {
                let rustc = rustc.to_string_lossy();
                eprintln!(
                    "Error: failed to parse output of `{} --version`: {}",
                    rustc, e
                );
                std::process::exit(1);
            }
        };

        match parse(&string) {
            ParseResult::Version(version) => break version,
            ParseResult::Clippy if !is_clippy_driver => {
                is_clippy_driver = true;
            }
            _ => {
                eprintln!(
                    "Error: unexpected output from `rustc --version`: {:?}",
                    string
                );
                std::process::exit(1);
            }
        }
    };

    if version.minor >= 38 {
        println!("cargo:rustc-cfg=ge_1_38_0");
    }
}

enum ParseResult {
    Clippy,
    Version(Version),
    None,
}

fn parse(string: &str) -> ParseResult {
    let last_line = string.lines().last().unwrap_or(string);
    let mut words = last_line.trim().split(' ').map(|x| x.trim());

    match words.next() {
        Some("rustc") => (),
        Some(word) if word.starts_with("clippy") => return ParseResult::Clippy,
        _ => return ParseResult::None,
    }

    parse_version(&mut words).map_or(ParseResult::None, ParseResult::Version)
}

fn parse_version(words: &mut dyn Iterator<Item = &str>) -> Option<Version> {
    let mut version_channel = words.next()?.split('-');
    let version = version_channel.next()?;

    let mut digits = version.split('.');
    let major = digits.next()?;
    if major != "1" {
        return None;
    }
    let minor = digits.next()?.parse().ok()?;
    let patch = digits.next().unwrap_or("0").parse().ok()?;

    Some(Version { minor, patch })
}
