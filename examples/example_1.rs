// cargo run --example example_1

use anyhow::Result;

fn main() -> Result<()> {
    println!("First a non existing path");
    let cross_path = crossplatform_path::CrossPathBuf::new(r#"c:\test\path"#)?;
    let cross_path = cross_path.join_relative("foo/bar")?.join_relative("one/two")?;
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

    println!("Second create a new directory and file");
    let cross_path = crossplatform_path::CrossPathBuf::new(r#"tmp/folder_1/file_1.txt"#)?;
    cross_path.write_str_to_file("content for testing")?;

    let content = cross_path.read_to_string()?;
    println!("content: {content}");

    Ok(())
}
