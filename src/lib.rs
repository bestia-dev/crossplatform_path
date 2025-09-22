// crossplatform_path/src/lib.rs

// The `main.rs` has all the stdin and stdout.
// The `lib.rs` must be in/out agnostic. That is the responsibility of the `main.rs`
// The `lib.rs` does not have any real code. All the code is in modules in separate files.
// The `lib.rs` has just the list of modules, it publishes module's functions or class for the caller
// and it has some global stuff like the Error enum.

// region: auto_md_to_doc_comments include README.md A //!
//! # crossplatform_path
//!
//! **Crossplatform Path Rust library**  
//! ***version: 0.0.16 date: 2025-09-22 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/crossplatform_path)***
//!
//!  ![maintained](https://img.shields.io/badge/maintained-green)
//!  ![work-in-progress](https://img.shields.io/badge/work_in_progress-yellow)
//!  ![rustlang](https://img.shields.io/badge/rustlang-orange)
//!
//!  
//!   [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/crossplatform_path/blob/main/LICENSE)
//!   [![Rust](https://github.com/bestia-dev/crossplatform_path/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/bestia-dev/crossplatform_path/)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-12-green.svg)]()
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-64-blue.svg)]()
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-10-purple.svg)]()
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-22-yellow.svg)]()
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)]()
//!
//! Hashtags: #maintained #ready-for-use #rustlang #automation #workflow  
//! My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).  
//!
//! ## Motivation
//!
//! I have a few Rust projects that are compiled for Linux and Windows. I need to save some paths inside a config file. Windows have a strange way to work with file/folder paths. I need a library to work in a neutral crossplatform way. Only at the last line of code I transform the neutral path into something that the current OS recognizes.
//!
//! There exist already some libraries for that: relative-path, typed-path, x-path, camino,...
//!
//! But it is fun to make something new and simple and very very opinionated.
//!
//! ## Opinionated to the max
//!
//! My opinions are probably not useful for all developers, but they work for me and probably for most.
//!
//! 1. The path will be strictly utf8. I know that there exists a rare possibility of the path being in some other strange format, but I will never encounter that with this library. Or at least, I will always avoid that.
//! 2. Unix and Linux have paths that look nice. Windows is the problem here. The crossplatform format will be very much like the Linux paths.
//! 3. The only path separator will be the universal '/'. If some '\' exists, it will be replaced by '/'. Linux allows the use of '\' inside a filename, but my opinion is that this is bad and should be avoided.
//! 4. Linux is very permissive. Only NULL and '/' are forbidden in path components. But it is a good idea to not allow special characters forbidden on Windows:  
//! < > : " / \ | ? *  
//! 0 (NULL byte)  
//! 0-31 (ASCII control characters)  
//! 5. Filenames cannot end in a space or dot.
//! 6. Not allow reserved filenames even with extensions:
//! CON, PRN, AUX, NUL  
//! COM1, COM2, COM3, COM4, COM5, COM6, COM7, COM8, COM9  
//! LPT1, LPT2, LPT3, LPT4, LPT5, LPT6, LPT7, LPT8, LPT9  
//! .  (special name referring to current directory)  
//! .. (special name referring to parent directory)  
//!
//! 7. Instead of the problematic Windows 'c:' or 'd:' drives, the neutral crossplatform format will be '/mnt/c' or '/mnt/d'  
//! 8. This special symbols are allowed and will be transformed for Windows:  
//! '~'    will be transformed into %UserProfile%
//! /tmp   will be transformed into %TEMP%
//! /mnt/c/ will be transformed into c:\
//! /mnt/d/ will be transformed into d:\
//!
//! ## Development details
//!
//! Read the development details in a separate md file:
//! [DEVELOPMENT.md](DEVELOPMENT.md)
//!
//! ## Releases changelog
//!
//! Read the releases changelog in a separate md file:
//! [RELEASES.md](RELEASES.md)
//!
//! ## TODO
//!
//! And code happily ever after...
//!
//! ## Open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
//! [//bestia.dev](https://bestia.dev)  
//! [//github.com/bestia-dev](https://github.com/bestia-dev)  
//! [//bestiadev.substack.com](https://bestiadev.substack.com)  
//! [//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

// The `main.rs` uses the `anyhow` error library.
// The `lib.rs` uses the `thiserror` library.
use thiserror::Error;

/// all possible library errors for `thiserror`
#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("Name `{0}` is already uppercase.")]
    Uppercase(String),
    #[error("Unknown error.")]
    Unknown,
}
