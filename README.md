<!-- markdownlint-disable MD041 -->
[//]: # (auto_md_to_doc_comments segment start A)

# crossplatform_path

[//]: # (auto_cargo_toml_to_md start)

**Crossplatform Path Rust library**  
***version: 1.1.3 date: 2025-09-25 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/crossplatform_path)***

 ![maintained](https://img.shields.io/badge/maintained-green)
 ![ready-for-use](https://img.shields.io/badge/ready_for_use-green)
 ![rustlang](https://img.shields.io/badge/rustlang-orange)

[//]: # (auto_cargo_toml_to_md end)

  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/crossplatform_path/blob/main/LICENSE)
  [![crates.io](https://img.shields.io/crates/v/crossplatform_path.svg)](https://crates.io/crates/crossplatform_path)
  [![Documentation](https://docs.rs/crossplatform_path/badge.svg)](https://docs.rs/crossplatform_path/)
  [![Rust](https://github.com/bestia-dev/crossplatform_path/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/bestia-dev/crossplatform_path/)
  ![crossplatform_path](https://bestia.dev/webpage_hit_counter/get_svg_image/1320456497.svg)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-80-green.svg)](https://github.com/bestia-dev/crossplatform_path/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-219-blue.svg)](https://github.com/bestia-dev/crossplatform_path/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-33-purple.svg)](https://github.com/bestia-dev/crossplatform_path/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-40-yellow.svg)](https://github.com/bestia-dev/crossplatform_path/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-303-orange.svg)](https://github.com/bestia-dev/crossplatform_path/)

[//]: # (auto_lines_of_code end)

Hashtags: #maintained #work-in-progress #rustlang  
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).  

## Motivation

I have a few Rust projects that are compiled for Linux and Windows. I need to save some paths inside a config file. Windows have a strange way to work with file/folder paths. I need a library to work in a neutral crossplatform way. Only at the last line of code I transform the neutral path into something that the current OS recognizes.

There exist already some libraries for that: relative-path, typed-path, x-path, camino,...

But it is fun to make something new and simple and very very opinionated.

## Opinionated to the max

My opinions are probably not useful for all developers, but they work for me and probably for most.

1. The path will be strictly utf8. I know that there exists a rare possibility of the path being in some other strange format, but I will never encounter that with this library. Or at least, I will always avoid that.
2. Unix and Linux have paths that look nice. Windows is the problem here. The crossplatform format will be very much like the Linux paths.
3. The only path separator will be the universal '/'. If some '\\' exists, it will be replaced by '/'. Linux allows the use of '\\' inside a filename, but my opinion is that this is bad and should be avoided.
4. Linux is very permissive. Only NULL and '/' are forbidden in path components. But it is a good idea to not allow special characters forbidden on Windows:  

    ```text
    < > : " / \\ | ? *
    0 (NULL byte)
    0-31 (ASCII control characters)  
    ```
  
5. Filenames cannot end in a space or dot.
6. Not allow reserved filenames even with extensions and foldernames:  
   CON, PRN, AUX, NUL  
   COM1, COM2, COM3, COM4, COM5, COM6, COM7, COM8, COM9  
   LPT1, LPT2, LPT3, LPT4, LPT5, LPT6, LPT7, LPT8, LPT9  
   These names are not really needed and will not be allowed:  
   .  (special name referring to current directory)  
   This have to be avoided because of traversal attacks:  
   .. (special name referring to parent directory)  

7. Instead of the problematic Windows 'c:' or 'd:' drives,  
   the neutral crossplatform format will be '/mnt/c' or '/mnt/d'  
   From Windows:  
   c:\\ will be transformed into /mnt/c/  
   d:\\ will be transformed into /mnt/d/  
8. This special symbols and root folders are allowed and will be transformed for Windows:  
   '~'    will be transformed into %UserProfile%  
   /tmp   will be transformed into %TEMP%  
9. Definitely some paths in one OS have absolutely no meaning in other OS, but these have to be avoided manually.

## Usage

```rust
// cargo add crossplatform_path

let cross_path = crossplatform_path::CrossPathBuf::new(r#"tmp\folder_1"#)?.join_relative(r#"file_1.txt"#)?;
println!("{cross_path}");

let linux_path_buf = cross_path.to_path_buf_nix();
println!("linux: {:?}", linux_path_buf);

let win_path_buf = cross_path.to_path_buf_win();
println!("windows: {:?}", win_path_buf);

let current_os_path_buf = cross_path.to_path_buf_current_os();
println!("current_os: {:?}", current_os_path_buf);

println!("exists: {}", cross_path.exists());
println!("is_dir: {}", cross_path.is_dir());
println!("is_file: {}", cross_path.is_file());

println!("parent: {}", cross_path.parent()?);
println!("file_name: {}", cross_path.file_name()?);
println!("file_stem: {}", cross_path.file_stem()?);
println!("extension: {}", cross_path.extension()?);

println!("write_str_to_file");
cross_path.write_str_to_file("content for testing")?;

let content = cross_path.read_to_string()?;
println!("read_to_string: {content}");


let cross_path = cross_path.add_start_slash()?.add_end_slash()?;
println!("add slashes {}", cross_path);

let cross_path = cross_path.trim_start_slash()?.trim_end_slash()?;
println!("trim slashes {}", cross_path);
   
# Ok::<(), crossplatform_path::LibraryError>(())
```

## Development details

Read the development details in a separate md file:
[DEVELOPMENT.md](DEVELOPMENT.md)

## Releases changelog

Read the releases changelog in a separate md file:
[RELEASES.md](RELEASES.md)

## TODO

Change panics into proper library errors.
And code happily ever after...

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
