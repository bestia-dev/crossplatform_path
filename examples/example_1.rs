// cargo run --example example_1

use anyhow::Error;

fn main() -> Result<(), Error> {
    let cross_path = crossplatform_path::CrossPathBuf::new(r#"c:\test\path"#)?;
    let cross_path = cross_path.join_relative("foo/bar")?.join_relative("one/two")?;
    println!("{cross_path}");

    let linux_path_buf = cross_path.to_path_buf_nix();
    println!("linux: {:?}", linux_path_buf);

    let win_path_buf = cross_path.to_path_buf_win();
    println!("windows: {:?}", win_path_buf);

    println!("exists: {}", cross_path.exists());
    println!("is_dir: {}", cross_path.is_dir());
    println!("is_file: {}", cross_path.is_file());

    if let Ok(_file) = std::fs::read_to_string(cross_path.to_path_buf_current_os()) {
        println!("File is found.");
    } else {
        println!("File is not found, but that is ok for this example.");
    }

    Ok(())
}
