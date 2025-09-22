// examples/example_1.rs

//! A simple example how to use the `lib.rs`
//! You can run it with `cargo run --example example_1`

#[allow(unused_imports)]
use crossplatform_path::*;

// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
#[allow(dead_code)]
pub const RED: &str = "\x1b[31m";
#[allow(dead_code)]
pub const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
#[allow(dead_code)]
pub const RESET: &str = "\x1b[0m";

/// example how to use format_hello_phrase() and format_upper_hello_phrase()
fn main() {
    println!("example");
}
