// crossplatform_path/src/lib.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # crossplatform_path
//!
//! **Crossplatform Path Rust library**  
//! ***version: 1.2.1 date: 2025-09-28 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/crossplatform_path)***
//!
//!  ![maintained](https://img.shields.io/badge/maintained-green)
//!  ![ready-for-use](https://img.shields.io/badge/ready_for_use-green)
//!  ![rustlang](https://img.shields.io/badge/rustlang-orange)
//!
//!   [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/crossplatform_path/blob/main/LICENSE)
//!   [![crates.io](https://img.shields.io/crates/v/crossplatform_path.svg)](https://crates.io/crates/crossplatform_path)
//!   [![Documentation](https://docs.rs/crossplatform_path/badge.svg)](https://docs.rs/crossplatform_path/)
//!   [![Rust](https://github.com/bestia-dev/crossplatform_path/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/bestia-dev/crossplatform_path/)
//!   ![crossplatform_path](https://bestia.dev/webpage_hit_counter/get_svg_image/1320456497.svg)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-80-green.svg)](https://github.com/bestia-dev/crossplatform_path/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-224-blue.svg)](https://github.com/bestia-dev/crossplatform_path/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-32-purple.svg)](https://github.com/bestia-dev/crossplatform_path/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-38-yellow.svg)](https://github.com/bestia-dev/crossplatform_path/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-303-orange.svg)](https://github.com/bestia-dev/crossplatform_path/)
//!
//! Hashtags: #maintained #work-in-progress #rustlang  
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
//! 7. Instead of the problematic Windows 'c:' or 'd:' drives,  
//!    the neutral crossplatform format will be '/mnt/c' or '/mnt/d'  
//!    From Windows:  
//!    c:\\ will be transformed into /mnt/c/  
//!    d:\\ will be transformed into /mnt/d/  
//! 8. This special symbols and root folders are allowed and will be transformed for Windows:  
//!    '~'    will be transformed into %UserProfile%  
//!    /tmp   will be transformed into %TEMP%  
//! 9. Definitely some paths in one OS have absolutely no meaning in other OS, but these have to be avoided manually.
//!
//! ## Usage
//!
//! ```rust
//! // cargo add crossplatform_path
//!
//! let cross_path = crossplatform_path::CrossPathBuf::new(r#"tmp\folder_1"#)?.join_relative(r#"file_1.txt"#)?;
//! println!("{cross_path}");
//!
//! let linux_path_buf = cross_path.to_path_buf_nix();
//! println!("linux: {:?}", linux_path_buf);
//!
//! let win_path_buf = cross_path.to_path_buf_win();
//! println!("windows: {:?}", win_path_buf);
//!
//! let current_os_path_buf = cross_path.to_path_buf_current_os();
//! println!("current_os: {:?}", current_os_path_buf);
//!
//! println!("exists: {}", cross_path.exists());
//! println!("is_dir: {}", cross_path.is_dir());
//! println!("is_file: {}", cross_path.is_file());
//!
//! println!("parent: {}", cross_path.parent()?);
//! println!("file_name: {}", cross_path.file_name()?);
//! println!("file_stem: {}", cross_path.file_stem()?);
//! println!("extension: {}", cross_path.extension()?);
//!
//! println!("write_str_to_file");
//! cross_path.write_str_to_file("content for testing")?;
//!
//! let content = cross_path.read_to_string()?;
//! println!("read_to_string: {content}");
//!
//!
//! let cross_path = cross_path.add_start_slash()?.add_end_slash()?;
//! println!("add slashes {}", cross_path);
//!
//! let cross_path = cross_path.trim_start_slash()?.trim_end_slash()?;
//! println!("trim slashes {}", cross_path);
//!    
//! # Ok::<(), crossplatform_path::Error>(())
//! ```
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

#[cfg(test)]
mod tests;

/// All possible library errors for `thiserror`.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(r#"The string {0} contains an invalid windows path character < > : " | ? * "#)]
    InvalidCharacter(String),
    #[error(r#"The string {0} contains forbidden ascii control character for windows path 0-31 "#)]
    ForbiddenAscii(String),
    #[error(r#"The path string {0} must not end with space or dot "#)]
    MustNotEndWith(String),
    #[error(r#"The path string {0} must not contain reserved words con, prn, aux, nul, com1-com9, lpt1-lpt9, . and .."#)]
    ReservedWords(String),
    #[error(r#"The parent of {0} does not exist."#)]
    NoParent(String),
    #[error(r#"The file_name of {0} does not exist."#)]
    NoFileName(String),
    #[error("I/O error: {path} {source}")]
    IoError {
        #[source]
        source: std::io::Error,
        path: String,
    },
    #[error("Unknown error.")]
    Unknown,
}

/// crossplatform_path::Result
///
/// `crossplatform_path::Result` is used with just one parameter.
/// Instead of the regular Result with second parameter,
/// that is always crossplatform_path::Error in this library.
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// CrossPathBuf stores Path in a Neutral Crossplatform format.  \
///
/// The neutral path is limited to valid utf8 strings.  \
/// This format can be stored in config files. It is "similar" to the Linux format, but not exactly equal.  \
/// When used for file operations, this Neutral format is converted into Linux or Windows format accordingly.  \
/// Some limitations exist for paths mostly because of Windows limitations:  \
/// forbidden characters < > : " / \\ | ? *  0 (NULL byte)  0-31 (ASCII control characters)  \
/// Filenames cannot end in a space or dot.  \
/// Separator is always slash. Backslash is replaced. Backslash must never be a part of a name or path component.  \
/// Must not contain reserved words con, prn, aux, nul, com1-com9, lpt1-lpt9, . and ..  \
/// If starts with windows c: or d: it is converted to /mnt/c or /mnt/d lowercase  
#[derive(Clone, Debug, PartialEq)]
pub struct CrossPathBuf {
    /// Path stored in a Neutral Crossplatform format.
    cross_path: String,
}

impl CrossPathBuf {
    /// Creates a new CrossPathBuf from &str.  \
    ///
    /// Path must be always utf8. Rust strings are guaranteed to be utf8.  \
    /// The input path will be tested that is somehow correct.  \
    /// It will be transformed from Windows into the crossplatform format. Linux format will stay mostly the same.  \
    /// The neutral path is limited to valid utf8 strings.  \
    /// This format can be stored in config files. It is "similar" to the Linux format, but not exactly equal.  \
    /// When used for file operations, this Neutral format is converted into Linux or Windows format accordingly.  \
    /// Some limitations exist for paths mostly because of Windows limitations:  \
    /// forbidden characters < > : " / \\ | ? *  0 (NULL byte)  0-31 (ASCII control characters)  \
    /// Filenames cannot end in a space or dot.  \
    /// Separator is always slash. Backslash is replaced. Backslash must never be a part of a name or path component.  \
    /// Must not contain reserved words con, prn, aux, nul, com1-com9, lpt1-lpt9, . and ..  \
    /// If start with windows c: or d: convert to /mnt/c or /mnt/d lowercase  
    pub fn new(str_path: &str) -> Result<Self> {
        // forbidden: < > : " / \\ | ? *  0 (NULL byte)  0-31 (ASCII control characters)
        // but : / and \\ are delimiters and can be used in a path fragment with multiple components.
        if str_path.contains("<")
            || str_path.contains(">")
            || str_path.contains(r#"""#)
            || str_path.contains("|")
            || str_path.contains("?")
            || str_path.contains("*")
        {
            return Err(Error::InvalidCharacter(str_path.to_string()));
        }
        // 0 (NULL byte) and  0-31 (ASCII control characters) 127 is DEL
        // Important: utf8 is always on a byte level compatible with ASCII7, under 127.
        for byte in str_path.bytes() {
            match byte {
                0x00..=0x1F | 0x7F => return Err(Error::ForbiddenAscii(str_path.to_string())),
                _ => (),
            }
        }
        //Filenames cannot end in a space or dot.
        if str_path.ends_with(" ") || str_path.ends_with(".") {
            return Err(Error::MustNotEndWith(str_path.to_string()));
        }

        // Separator is always slash. Backslash is replaced. Backslash must never be a part of a name or path component.
        // trim: leading and trailing whitespace removed
        let mut cross_path = str_path.trim().replace(r#"\"#, "/");

        // 6. Not allow reserved filenames even with extensions and foldernames:
        // Windows path is case insensitive, so I must check insensitive. I will use to_lowercase.
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
            return Err(Error::ReservedWords(str_path.to_string()));
        }

        // If start with windows c: or d: convert to /mnt/c or /mnt/d lowercase
        let mut iter = cross_path.chars();
        if let Some(first) = iter.next()
            && let Some(second) = iter.next()
            && second == ':'
        {
            cross_path = format!("/mnt/{}{}", first.to_lowercase(), iter.as_str());
        }

        // Forbidden character, except for windows drive
        if cross_path.contains(":") {
            return Err(Error::InvalidCharacter(cross_path));
        }

        // Forbidden double slash
        if cross_path.contains("//") {
            return Err(Error::InvalidCharacter(cross_path));
        }

        Ok(CrossPathBuf { cross_path })
    }

    /// Converts crossplatform path into Windows path.  \
    ///
    /// '~'     will be transformed into home  \
    /// /mnt/c/ will be transformed into c:\\  \
    /// /mnt/d/ will be transformed into d:\\  \
    /// /tmp    will be transformed into %TEMP%  
    pub fn to_path_buf_win(&self) -> std::path::PathBuf {
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

    /// Converts crossplatform path into Linux path.  \
    ///
    /// '~'    will be transformed into home  
    pub fn to_path_buf_nix(&self) -> std::path::PathBuf {
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

    /// Converts crossplatform path into current OS path.  \
    ///
    /// '~'     will be transformed into home  \
    /// /mnt/c/ will be transformed into c:\\  \
    /// /mnt/d/ will be transformed into d:\\  \
    /// /tmp    will be transformed into %TEMP%  
    pub fn to_path_buf_current_os(&self) -> std::path::PathBuf {
        if cfg!(windows) {
            self.to_path_buf_win()
        } else {
            self.to_path_buf_nix()
        }
    }

    /// Returns the crossplatform str for use in Display and store into config files.
    pub fn as_str(&self) -> &str {
        &self.cross_path
    }

    /// Returns `true` if the path points at an existing entity.
    pub fn exists(&self) -> bool {
        if cfg!(windows) {
            self.to_path_buf_win().exists()
        } else {
            self.to_path_buf_nix().exists()
        }
    }

    /// Returns `true` if the path exists on disk and is pointing at a regular file.
    pub fn is_file(&self) -> bool {
        if cfg!(windows) {
            self.to_path_buf_win().is_file()
        } else {
            self.to_path_buf_nix().is_file()
        }
    }

    /// Returns `true` if the path exists on disk and is pointing at a directory.
    pub fn is_dir(&self) -> bool {
        if cfg!(windows) {
            self.to_path_buf_win().is_dir()
        } else {
            self.to_path_buf_nix().is_dir()
        }
    }

    /// Joins two paths and returns a new CrossPathBuf to allow function chaining.  \
    ///
    /// It works differently from the original Rust join() where if the second path is absolute, it overwrites the first path.  \
    /// Here the second path is always relative and is added to the first path.
    pub fn join_relative(&self, str_path: &str) -> Result<Self> {
        let second_path = CrossPathBuf::new(str_path)?;
        let cross_path = format!(
            "{}/{}",
            self.cross_path.trim_end_matches("/"),
            second_path.as_str().trim_start_matches("/")
        );
        Ok(CrossPathBuf { cross_path })
    }

    /// Reads the entire contents of a file into a string.  \
    ///
    /// This is a convenience function based on std::fs::read_to_string  
    pub fn read_to_string(&self) -> Result<String> {
        match std::fs::read_to_string(self.to_path_buf_current_os()) {
            Ok(content) => Ok(content),
            Err(err) => Err(Error::IoError {
                source: err,
                path: self.cross_path.to_string(),
            }),
        }
    }

    /// Writes a slice as the entire contents of a file.  \
    ///
    /// This function will create a file if it does not exist, and will entirely replace its contents if it does.  \
    /// It creates the full path directory, if path does not exist.  
    pub fn write_str_to_file(&self, content: &str) -> Result<()> {
        self.create_dir_all_for_file()?;
        match std::fs::write(self.to_path_buf_current_os(), content) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::IoError {
                source: err,
                path: self.cross_path.clone(),
            }),
        }
    }

    /// Recursively create this path as directory and all of its parent components if they are missing.  \
    ///
    /// The cross_path must represent a directory and not a file for this command.
    /// This function is not atomic. If it returns an error, any parent components it was able to create will remain.   
    pub fn create_dir_all(&self) -> Result<()> {
        match std::fs::create_dir_all(self.to_path_buf_current_os()) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::IoError {
                source: err,
                path: self.cross_path.clone(),
            }),
        }
    }

    /// Recursively create the parent directory of a file and all of its parent components if they are missing.  \
    ///
    /// The cross_path must represent a file. The parent directory will be created.
    /// This function is not atomic. If it returns an error, any parent components it was able to create will remain.   
    pub fn create_dir_all_for_file(&self) -> Result<()> {
        let path = self.to_path_buf_current_os();
        let parent = path.parent().ok_or_else(|| Error::NoParent(self.cross_path.clone()))?;
        match std::fs::create_dir_all(parent) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::IoError {
                source: err,
                path: self.cross_path.to_string(),
            }),
        }
    }

    /// Returns a CrossPathBuf without leading start slash (repeatedly removed).  
    pub fn trim_start_slash(&self) -> Result<Self> {
        let cross_path = self.cross_path.trim_start_matches('/').trim().to_string();
        Ok(CrossPathBuf { cross_path })
    }

    /// Returns a CrossPathBuf without trailing end slash (repeatedly removed).  
    pub fn trim_end_slash(&self) -> Result<Self> {
        let cross_path = self.cross_path.trim_end_matches('/').trim().to_string();
        Ok(CrossPathBuf { cross_path })
    }

    /// Returns a CrossPathBuf with one leading start slash.  
    pub fn add_start_slash(&self) -> Result<Self> {
        let cross_path = format!("/{}", self.cross_path.trim_start_matches('/').trim());
        Ok(CrossPathBuf { cross_path })
    }

    /// Returns a CrossPathBuf with one trailing end slash.  
    pub fn add_end_slash(&self) -> Result<Self> {
        let cross_path = format!("{}/", self.cross_path.trim_end_matches('/').trim());
        Ok(CrossPathBuf { cross_path })
    }

    /// Returns the final component of the Path, if there is one.
    ///
    /// If the path is a normal file, this is the file name.
    /// If it's the path of a directory, this is the directory name.
    pub fn file_name(&self) -> Result<String> {
        let file_name = self
            .to_path_buf_current_os()
            .file_name()
            .ok_or_else(|| Error::NoFileName(self.cross_path.clone()))?
            .to_string_lossy()
            .to_string();
        Ok(file_name)
    }

    /// Extracts the extension (without the leading dot), if possible.  
    pub fn extension(&self) -> Result<String> {
        let file_name = self
            .to_path_buf_current_os()
            .extension()
            .ok_or_else(|| Error::NoFileName(self.cross_path.clone()))?
            .to_string_lossy()
            .to_string();
        Ok(file_name)
    }

    /// Extracts the stem (non-extension) portion of file_name (the final component of the Path).
    pub fn file_stem(&self) -> Result<String> {
        let file_name = self
            .to_path_buf_current_os()
            .file_stem()
            .ok_or_else(|| Error::NoFileName(self.cross_path.clone()))?
            .to_string_lossy()
            .to_string();
        Ok(file_name)
    }

    /// Returns the Path without its final component, if there is one.
    pub fn parent(&self) -> Result<Self> {
        CrossPathBuf::new(
            &self
                .to_path_buf_current_os()
                .parent()
                .ok_or_else(|| Error::NoParent(self.cross_path.clone()))?
                .to_string_lossy(),
        )
    }
}

/// Method display() is used in format!("{}").
impl std::fmt::Display for CrossPathBuf {
    /// Method display() is used in format!("{}").
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self.as_str(), f)
    }
}

/// CrossPathBuf from() and into() are useful in places where PathBuf is needed.
impl From<CrossPathBuf> for std::path::PathBuf {
    /// CrossPathBuf from() and into() are useful in places where PathBuf is needed.
    fn from(cross_path: CrossPathBuf) -> Self {
        cross_path.to_path_buf_current_os()
    }
}

// TODO: is it possible to impl AsRef<Path>?
// many functions accept AsRef<Path>
// problem: I cannot create a PathBuf inside as_ref() because
// then cannot return value referencing temporary value
/* impl AsRef<std::path::Path> for CrossPathBuf {
    #[inline]
    fn as_ref(&self) -> &std::path::Path {
        self.to_current_os_path_buf().as_path()
    }
}
 */
