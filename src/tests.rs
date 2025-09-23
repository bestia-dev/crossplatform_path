use super::*;

#[test]
fn test_01_slash() {
    let cross_path = CrossPathBuf::new("test/path").unwrap();
    assert_eq!(cross_path.as_str(), "test/path");
    assert_eq!(cross_path.to_path_buf_nix().to_string_lossy(), "test/path");
    assert_eq!(cross_path.to_path_buf_win().to_string_lossy(), "test/path");
}

#[test]
fn test_02_backslash() {
    let cross_path = CrossPathBuf::new(r#"test\path"#).unwrap();
    assert_eq!(cross_path.as_str(), "test/path");
    assert_eq!(cross_path.to_path_buf_nix().to_string_lossy(), "test/path");
    assert_eq!(cross_path.to_path_buf_win().to_string_lossy(), "test/path");
}

#[test]
fn test_03_c_drive_backslash() {
    let cross_path = CrossPathBuf::new(r#"c:\test\path"#).unwrap();
    assert_eq!(cross_path.as_str(), "/mnt/c/test/path");
    assert_eq!(cross_path.to_path_buf_nix().to_string_lossy(), "/mnt/c/test/path");
    assert_eq!(cross_path.to_path_buf_win().to_string_lossy(), "c:/test/path");
}

#[test]
fn test_03_c_drive_slash() {
    let cross_path = CrossPathBuf::new(r#"c:/test/path"#).unwrap();
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
    let cross_path = CrossPathBuf::new(r#"~/test/path"#).unwrap();
    assert_eq!(cross_path.as_str(), "~/test/path");
    assert_eq!(cross_path.to_path_buf_nix().to_string_lossy(), "/home/rustdevuser/test/path");
    assert_eq!(cross_path.to_path_buf_win().to_string_lossy(), "/home/rustdevuser/test/path");
}

/// test join_relative
#[test]
fn test_06_join_relative() {
    let cross_path = CrossPathBuf::new(r#"test/path"#).unwrap();
    let cross_path = cross_path.join_relative("foo/bar").unwrap().join_relative("foo2/bar2").unwrap();
    assert_eq!(cross_path.as_str(), "test/path/foo/bar/foo2/bar2");
    assert_eq!(cross_path.to_path_buf_nix().to_string_lossy(), "test/path/foo/bar/foo2/bar2");
    assert_eq!(cross_path.to_path_buf_win().to_string_lossy(), "test/path/foo/bar/foo2/bar2");
}
