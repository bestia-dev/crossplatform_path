// crossplatform_path/src/lib.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # crossplatform_path
//!
//! **Crossplatform Path Rust library**  
//! ***version: 1.0.2 date: 2025-09-23 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/crossplatform_path)***
//!
//!  ![maintained](https://img.shields.io/badge/maintained-green)
//!  ![work-in-progress](https://img.shields.io/badge/work_in_progress-yellow)
//!  ![rustlang](https://img.shields.io/badge/rustlang-orange)
//!
//!   [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/crossplatform_path/blob/main/LICENSE)
//!   [![Rust](https://github.com/bestia-dev/crossplatform_path/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/bestia-dev/crossplatform_path/)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-58-green.svg)](https://github.com/bestia-dev/crossplatform_path/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-142-blue.svg)](https://github.com/bestia-dev/crossplatform_path/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-28-purple.svg)](https://github.com/bestia-dev/crossplatform_path/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-23-yellow.svg)](https://github.com/bestia-dev/crossplatform_path/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-182-orange.svg)](https://github.com/bestia-dev/crossplatform_path/)
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

// The `lib.rs` uses the `thiserror` library.
use thiserror::Error;

/// all possible library errors for `thiserror`
#[derive(Error, Debug, PartialEq)]
pub enum LibraryError {
    #[error(r#"The string {0} contains an invalid windows path character < > : " | ? * "#)]
    InvalidCharacter(String),
    #[error(r#"The string {0} contains forbidden ascii control character for windows path 0-31 "#)]
    ForbiddenAscii(String),
    #[error(r#"The path string {0} must not end with space or dot "#)]
    MustNotEndWith(String),
    #[error(r#"The path string {0} must not contain reserved words con, prn, aux, nul, com1-com9, lpt1-lpt9, . and .."#)]
    ReservedWords(String),
    #[error("Unknown error.")]
    Unknown,
}

/// Crossplatform PathBuf stores Path in a Neutral Crossplatform format
///
/// The neutral path is limited to valid utf8 strings.
/// This format can be stored in config files. It is "similar" to the Linux format, but not exactly equal.
/// When used for file operations, this Neutral format is converted into Linux or Windows format accordingly.
/// Some limitations exist for paths mostly because of Windows limitations:
/// forbidden characters < > : " / \\ | ? *  0 (NULL byte)  0-31 (ASCII control characters)
/// Filenames cannot end in a space or dot.
/// separator is always slash. Backslash is replaced. Backslash must never be a part of a name or path component.
/// must not contain reserved words con, prn, aux, nul, com1-com9, lpt1-lpt9, . and ..
/// if starts with windows c: or d: it is converted to /mnt/c or /mnt/d lowercase
#[derive(Clone, Debug, PartialEq)]
pub struct CrossPathBuf {
    cross_path: String,
}

impl CrossPathBuf {
    /// Create a new CrossPlatform path from a str
    /// Path must be always utf8. Rust strings are guaranteed to be utf8.
    /// The input path will be tested that is somehow correct.
    /// It will be transformed from Windows into the crossplatform format. Linux format will stay mostly the same.
    /// The neutral path is limited to valid utf8 strings.
    /// This format can be stored in config files. It is "similar" to the Linux format, but not exactly equal.
    /// When used for file operations, this Neutral format is converted into Linux or Windows format accordingly.
    /// Some limitations exist for paths mostly because of Windows limitations:
    /// forbidden characters < > : " / \\ | ? *  0 (NULL byte)  0-31 (ASCII control characters)
    /// Filenames cannot end in a space or dot.
    /// separator is always slash. Backslash is replaced. Backslash must never be a part of a name or path component.
    /// must not contain reserved words con, prn, aux, nul, com1-com9, lpt1-lpt9, . and ..
    /// if start with windows c: or d: convert to /mnt/c or /mnt/d lowercase
    pub fn new(str_path: &str) -> Result<Self, LibraryError> {
        // forbidden: < > : " / \\ | ? *  0 (NULL byte)  0-31 (ASCII control characters)
        // but : / and \\ are delimiters and can be used in a path fragment with multiple components.
        if str_path.contains("<")
            || str_path.contains(">")
            || str_path.contains(r#"""#)
            || str_path.contains("|")
            || str_path.contains("?")
            || str_path.contains("*")
        {
            return Err(LibraryError::InvalidCharacter(str_path.to_string()));
        }
        // 0 (NULL byte) and  0-31 (ASCII control characters) 127 is DEL
        // Important: utf8 is always on a byte level compatible with ASCII7, under 127.
        for byte in str_path.bytes() {
            match byte {
                0x00..=0x1F | 0x7F => return Err(LibraryError::ForbiddenAscii(str_path.to_string())),
                _ => (),
            }
        }
        //Filenames cannot end in a space or dot.
        if str_path.ends_with(" ") || str_path.ends_with(".") {
            return Err(LibraryError::MustNotEndWith(str_path.to_string()));
        }

        // separator is always slash. Backslash is replaced. Backslash must never be a part of a name or path component.
        let mut cross_path = str_path.replace(r#"\"#, "/");

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
            return Err(LibraryError::ReservedWords(str_path.to_string()));
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
            return Err(LibraryError::InvalidCharacter(cross_path));
        }

        // forbidden double slash
        if cross_path.contains("//") {
            return Err(LibraryError::InvalidCharacter(cross_path));
        }

        Ok(CrossPathBuf { cross_path })
    }

    /// convert crossplatform path into Windows path
    /// '~'    will be transformed into home
    /// /mnt/c/ will be transformed into c:\\
    /// /mnt/d/ will be transformed into d:\\
    /// /tmp   will be transformed into %TEMP%
    pub fn to_win_path_buf(&self) -> std::path::PathBuf {
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
        use std::str::FromStr;
        std::path::PathBuf::from_str(&win_path).expect("PathBuf::from_str() returns Infallible error. Therefore the error cannot occur.")
    }

    /// convert crossplatform path into Linux path
    /// '~'    will be transformed into home
    pub fn to_nix_path_buf(&self) -> std::path::PathBuf {
        let mut nix_path = self.cross_path.clone();
        // '~'    will be transformed into home
        if nix_path.starts_with("~")
            && let Some(home) = std::env::home_dir()
        {
            nix_path = format!("{}{}", home.to_string_lossy(), nix_path.trim_start_matches("~"));
        }
        use std::str::FromStr;
        std::path::PathBuf::from_str(&nix_path).expect("PathBuf::from_str() returns Infallible error. Therefore the error cannot occur.")
    }

    /// convert crossplatform path into current OS path
    /// '~'    will be transformed into home
    /// /mnt/c/ will be transformed into c:\\
    /// /mnt/d/ will be transformed into d:\\
    /// /tmp   will be transformed into %TEMP%
    pub fn to_current_os_path_buf(&self) -> std::path::PathBuf {
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

    /// Returns `true` if the path exists on disk and is pointing at a regular file.
    pub fn is_file(&self) -> bool {
        if cfg!(windows) {
            self.to_win_path_buf().is_file()
        } else {
            self.to_nix_path_buf().is_file()
        }
    }

    /// Returns `true` if the path exists on disk and is pointing at a directory.
    pub fn is_dir(&self) -> bool {
        if cfg!(windows) {
            self.to_win_path_buf().is_dir()
        } else {
            self.to_nix_path_buf().is_dir()
        }
    }

    /// Joins two paths and returns a new CrossPath to allow function chaining
    ///
    /// It works differently from the original Rust join() where if the second path is absolute, it overwrites the first path.
    /// Here the second path is always relative and is added to the first path
    pub fn join_relative(&self, str_path: &str) -> Result<Self, LibraryError> {
        let second_path = CrossPathBuf::new(str_path)?;
        let cross_path = format!(
            "{}/{}",
            self.cross_path.trim_end_matches("/"),
            second_path.as_str().trim_start_matches("/")
        );
        Ok(CrossPathBuf { cross_path })
    }
}

/// display() is used in format!("{}")
impl std::fmt::Display for CrossPathBuf {
    /// display() is used in format!("{}")
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.as_str(), f)
    }
}

/// from() and into() are useful in places where PathBuf is needed
impl From<CrossPathBuf> for std::path::PathBuf {
    /// from() and into() are useful in places where PathBuf is needed
    fn from(cross_path: CrossPathBuf) -> Self {
        cross_path.to_current_os_path_buf()
    }
}
