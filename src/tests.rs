use super::*;

#[test]
fn test_01_slash() {
    let cross_path = CrossPathBuf::new("test/path").expect("test");
    assert_eq!(cross_path.as_str(), "test/path");
    assert_eq!(cross_path.to_path_buf_nix().to_string_lossy(), "test/path");
    assert_eq!(cross_path.to_path_buf_win().to_string_lossy(), "test/path");
}

#[test]
fn test_02_backslash() {
    let cross_path = CrossPathBuf::new(r#"test\path"#).expect("test");
    assert_eq!(cross_path.as_str(), "test/path");
    assert_eq!(cross_path.to_path_buf_nix().to_string_lossy(), "test/path");
    assert_eq!(cross_path.to_path_buf_win().to_string_lossy(), "test/path");
}

#[test]
fn test_03_c_drive_backslash() {
    let cross_path = CrossPathBuf::new(r#"c:\test\path"#).expect("test");
    assert_eq!(cross_path.as_str(), "/mnt/c/test/path");
    assert_eq!(cross_path.to_path_buf_nix().to_string_lossy(), "/mnt/c/test/path");
    assert_eq!(cross_path.to_path_buf_win().to_string_lossy(), "c:/test/path");
}

#[test]
fn test_03_c_drive_slash() {
    let cross_path = CrossPathBuf::new(r#"c:/test/path"#).expect("test");
    assert_eq!(cross_path.as_str(), "/mnt/c/test/path");
    assert_eq!(cross_path.to_path_buf_nix().to_string_lossy(), "/mnt/c/test/path");
    assert_eq!(cross_path.to_path_buf_win().to_string_lossy(), "c:/test/path");
}

#[test]
fn test_04_invalid_character() {
    match CrossPathBuf::new(r#"c:\test:\path"#) {
        Err(err) => assert_eq!(
            err.to_string(),
            r#"The string /mnt/c/test:/path contains an invalid windows path character < > : " | ? * "#
        ),
        Ok(_path) => (),
    }
}

#[test]
fn test_05_home() {
    let cross_path = CrossPathBuf::new(r#"~/test/path"#).expect("test");
    assert_eq!(cross_path.as_str(), "~/test/path");
    assert_eq!(cross_path.to_path_buf_nix().to_string_lossy(), "/home/rustdevuser/test/path");
    assert_eq!(cross_path.to_path_buf_win().to_string_lossy(), "/home/rustdevuser/test/path");
}

/// test join_relative
#[test]
fn test_06_join_relative() {
    let cross_path = CrossPathBuf::new(r#"test/path"#).expect("test");
    let cross_path = cross_path
        .join_relative("foo/bar")
        .expect("test")
        .join_relative("foo2/bar2")
        .expect("test");
    assert_eq!(cross_path.as_str(), "test/path/foo/bar/foo2/bar2");
    assert_eq!(cross_path.to_path_buf_nix().to_string_lossy(), "test/path/foo/bar/foo2/bar2");
    assert_eq!(cross_path.to_path_buf_win().to_string_lossy(), "test/path/foo/bar/foo2/bar2");
}

#[test]
fn test_07_trim_add() {
    let cross_path = CrossPathBuf::new(r#"test/path"#).expect("test");
    let cross_path = cross_path.add_start_slash().expect("test").add_end_slash().expect("test");
    assert_eq!(cross_path.as_str(), "/test/path/");
    let cross_path = cross_path.trim_start_slash().expect("test").trim_end_slash().expect("test");
    assert_eq!(cross_path.as_str(), "test/path");
}

#[test]
fn test_07_file_name() {
    let cross_path = CrossPathBuf::new(r#"foo/bar.txt"#).expect("test");
    assert_eq!(cross_path.file_name().expect("test"), "bar.txt");
    assert_eq!(cross_path.file_stem().expect("test"), "bar");
    assert_eq!(cross_path.extension().expect("test"), "txt");

    let cross_path = cross_path.replace_extension("md").expect("test");
    assert_eq!(cross_path.file_name().expect("test"), "bar.md");
    assert_eq!(cross_path.file_stem().expect("test"), "bar");
    assert_eq!(cross_path.extension().expect("test"), "md");
}
