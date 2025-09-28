// cargo run --example example_1

fn main() -> anyhow::Result<()> {
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

    Ok(())
}
