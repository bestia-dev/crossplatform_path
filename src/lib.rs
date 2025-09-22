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
//! ***version: 0.0.30 date: 2025-09-22 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/crossplatform_path)***
//!
//!  ![maintained](https://img.shields.io/badge/maintained-green)
//!  ![work-in-progress](https://img.shields.io/badge/work_in_progress-yellow)
//!  ![rustlang](https://img.shields.io/badge/rustlang-orange)
//!
//!   [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/crossplatform_path/blob/main/LICENSE)
//!   [![Rust](https://github.com/bestia-dev/crossplatform_path/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/bestia-dev/crossplatform_path/)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-52-green.svg)](https://github.com/bestia-dev/crossplatform_path/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-102-blue.svg)](https://github.com/bestia-dev/crossplatform_path/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-33-purple.svg)](https://github.com/bestia-dev/crossplatform_path/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-23-yellow.svg)](https://github.com/bestia-dev/crossplatform_path/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-160-orange.svg)](https://github.com/bestia-dev/crossplatform_path/)
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
//! 3. The only path separator will be the universal '/'. If some '\\' exists, it will be replaced by '/'. Linux allows the use of '\\' inside a filename, but my opinion is that this is bad and should be avoided.
//! 4. Linux is very permissive. Only NULL and '/' are forbidden in path components. But it is a good idea to not allow special characters forbidden on Windows:  
//!
//!     ```text
//!     < > : " / \\ | ? *
//!     0 (NULL byte)
//!     0-31 (ASCII control characters)  
//!     ```
//!   
//! 5. Filenames cannot end in a space or dot.
//! 6. Not allow reserved filenames even with extensions and foldernames:
//!    CON, PRN, AUX, NUL  
//!    COM1, COM2, COM3, COM4, COM5, COM6, COM7, COM8, COM9  
//!    LPT1, LPT2, LPT3, LPT4, LPT5, LPT6, LPT7, LPT8, LPT9  
//!    These names are not really needed and will not be allowed:
//!    .  (special name referring to current directory)  
//!    This have to be avoided because of traversal attacks:
//!    .. (special name referring to parent directory)  
//!
//! 7. Instead of the problematic Windows 'c:' or 'd:' drives, the neutral crossplatform format will be '/mnt/c' or '/mnt/d'  
//!    On Windows:
//!    /mnt/c/ will be transformed into c:\\  
//!    /mnt/d/ will be transformed into d:\\  
//! 8. This special symbols and root folders are allowed and will be transformed for Windows:  
//!    '~'    will be transformed into %UserProfile%  
//!    /tmp   will be transformed into %TEMP%  
//! 9. Definitely some paths in one OS have absolutely no meaning in other OS, but these have to be avoided manually.
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
//! Change panics into proper library errors.
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

#[cfg(test)]
mod tests;

use std::{fmt, path::PathBuf, str::FromStr};

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

#[derive(Clone, Default)]
pub struct CrossPathBuf {
    cross_path: String,
}

impl CrossPathBuf {
    /// Path must be always utf8. Rust strings are guaranteed to be utf8.
    /// The input path will be tested that is somehow correct.
    /// It will be transformed from Windows into the crossplatform format. Linux format will stay the same.
    /// c:\foo\bar will be transformed into /mnt/c/foo/bar
    #[must_use]
    pub fn new(string_path: String) -> Self {
        // forbidden: < > : " / \\ | ? *  0 (NULL byte)  0-31 (ASCII control characters)
        // but : / and \\ are delimiters and can be used in a path fragment with multiple components.
        if string_path.contains("<")
            || string_path.contains(">")
            || string_path.contains(r#"""#)
            || string_path.contains("|")
            || string_path.contains("?")
            || string_path.contains("*")
        {
            panic!(
                r#"The string {} contains an invalid windows path character < > : " | ? * "#,
                string_path
            )
        }
        // 0 (NULL byte) and  0-31 (ASCII control characters) 127 is DEL
        // Important: utf8 is always on a byte level compatible with ASCII7, under 127.
        for byte in string_path.bytes() {
            match byte {
                0x00..=0x1F | 0x7F => panic!(
                    r#"The string {} contains forbidden ascii control character for windows path 0-31 "#,
                    string_path
                ),
                _ => (),
            }
        }
        //Filenames cannot end in a space or dot.
        if string_path.ends_with(" ") || string_path.ends_with(".") {
            panic!(r#"The path string {} must not end with space or dot "#, string_path)
        }

        // separator is always slash. Backslash is replaced. Backslash must never be a part of a name or path component.
        let mut cross_path = string_path.replace(r#"\"#, "/");

        // 6. Not allow reserved filenames even with extensions and foldernames:
        // windows path is case insensitive, so I must check insensitive. I will use to_lowercase.
        // CON, PRN, AUX, NUL
        // COM1, COM2, COM3, COM4, COM5, COM6, COM7, COM8, COM9
        // LPT1, LPT2, LPT3, LPT4, LPT5, LPT6, LPT7, LPT8, LPT9
        // These names are not really needed and will not be allowed:
        // .  (special name referring to current directory)
        // This have to be avoided because of traversal attacks:
        // .. (special name referring to parent directory)
        // I need to delimit start and end with / to use contains.
        let delimited_str_path = format!("/{}/", cross_path.trim_start_matches("/").trim_end_matches("/").to_lowercase());
        if delimited_str_path.contains("/con/")
            || delimited_str_path.contains("/prn/")
            || delimited_str_path.contains("/aux/")
            || delimited_str_path.contains("/nul/")
            || delimited_str_path.contains("/com1/")
            || delimited_str_path.contains("/com2/")
            || delimited_str_path.contains("/com3/")
            || delimited_str_path.contains("/com4/")
            || delimited_str_path.contains("/com5/")
            || delimited_str_path.contains("/com6/")
            || delimited_str_path.contains("/com7/")
            || delimited_str_path.contains("/com8/")
            || delimited_str_path.contains("/com9/")
            || delimited_str_path.contains("/lpt1/")
            || delimited_str_path.contains("/lpt2/")
            || delimited_str_path.contains("/lpt3/")
            || delimited_str_path.contains("/lpt4/")
            || delimited_str_path.contains("/lpt5/")
            || delimited_str_path.contains("/lpt6/")
            || delimited_str_path.contains("/lpt7/")
            || delimited_str_path.contains("/lpt8/")
            || delimited_str_path.contains("/lpt9/")
            || delimited_str_path.contains("/./")
            || delimited_str_path.contains("/../")
        {
            panic!(
                r#"The path string {} must not contain reserved words con, prn, aux, nul, com1-com9, lpt1-lpt9, . and .."#,
                string_path
            )
        }

        // if start with windows c: or d: convert to /mnt/c or /mnt/d lowercase
        let mut iter = cross_path.chars();
        if let Some(first) = iter.next()
            && let Some(second) = iter.next()
            && second == ':'
        {
            cross_path = format!("/mnt/{}{}", first.to_lowercase(), iter.as_str());
        }

        // forbidden character, except for windows drive
        if cross_path.contains(":") {
            panic!(
                r#"The string {} contains an invalid windows path character < > : " | ? * "#,
                string_path
            )
        }

        CrossPathBuf { cross_path }
    }

    /// convert crossplatform path into Windows path
    pub fn to_win_path_buf(&self) -> PathBuf {
        let mut win_path = self.cross_path.clone();
        // '~'    will be transformed into home
        if win_path.starts_with("~")
            && let Some(home) = std::env::home_dir()
        {
            win_path = format!("{}{}", home.to_string_lossy(), win_path.trim_start_matches("~"));
        }
        // /mnt/c/ will be transformed into c:\\
        // /mnt/d/ will be transformed into d:\\
        if win_path.starts_with("/mnt/") {
            win_path = win_path.trim_start_matches("/mnt/").to_string();
            win_path.insert(1, ':');
        }
        // /tmp   will be transformed into %TEMP%
        if win_path.starts_with("/tmp") {
            let tmp_dir = std::env::temp_dir();
            win_path = format!("{}{}", tmp_dir.to_string_lossy(), win_path.trim_start_matches("/tmp"));
        }

        PathBuf::from_str(&win_path).unwrap()
    }

    /// convert crossplatform path into Linux path
    pub fn to_nix_path_buf(&self) -> PathBuf {
        let mut nix_path = self.cross_path.clone();
        // '~'    will be transformed into home
        if nix_path.starts_with("~")
            && let Some(home) = std::env::home_dir()
        {
            nix_path = format!("{}{}", home.to_string_lossy(), nix_path.trim_start_matches("~"));
        }
        PathBuf::from_str(&nix_path).unwrap()
    }

    /// convert crossplatform path into current OS path
    pub fn to_current_os_path_buf(&self) -> PathBuf {
        if cfg!(windows) {
            self.to_win_path_buf()
        } else {
            self.to_nix_path_buf()
        }
    }

    /// return as crossplatform str for use in Display and Debug and store into config files
    pub fn as_str(&self) -> &str {
        &self.cross_path
    }

    /// Returns `true` if the path points at an existing entity.
    pub fn exists(&self) -> bool {
        if cfg!(windows) {
            self.to_win_path_buf().exists()
        } else {
            self.to_nix_path_buf().exists()
        }
    }
}

impl fmt::Debug for CrossPathBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl fmt::Display for CrossPathBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}
