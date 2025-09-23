use super::*;

#[test]
fn test_01_slash() {
    let cross_path = CrossPathBuf::new("test/path").unwrap();
    assert_eq!(cross_path.as_str(), "test/path");
    assert_eq!(cross_path.to_nix_path_buf().to_string_lossy(), "test/path");
    assert_eq!(cross_path.to_win_path_buf().to_string_lossy(), "test/path");
}

#[test]
fn test_02_backslash() {
    let cross_path = CrossPathBuf::new(r#"test\path"#).unwrap();
    assert_eq!(cross_path.as_str(), "test/path");
    assert_eq!(cross_path.to_nix_path_buf().to_string_lossy(), "test/path");
    assert_eq!(cross_path.to_win_path_buf().to_string_lossy(), "test/path");
}

#[test]
fn test_03_c_drive_backslash() {
    let cross_path = CrossPathBuf::new(r#"c:\test\path"#).unwrap();
    assert_eq!(cross_path.as_str(), "/mnt/c/test/path");
    assert_eq!(cross_path.to_nix_path_buf().to_string_lossy(), "/mnt/c/test/path");
    assert_eq!(cross_path.to_win_path_buf().to_string_lossy(), "c:/test/path");
}

#[test]
fn test_03_c_drive_slash() {
    let cross_path = CrossPathBuf::new(r#"c:/test/path"#).unwrap();
    assert_eq!(cross_path.as_str(), "/mnt/c/test/path");
    assert_eq!(cross_path.to_nix_path_buf().to_string_lossy(), "/mnt/c/test/path");
    assert_eq!(cross_path.to_win_path_buf().to_string_lossy(), "c:/test/path");
}

#[test]
fn test_04_invalid_character() {
    let result = CrossPathBuf::new(r#"c:\test:\path"#);
    assert_eq!(result.err(), Some(LibraryError::InvalidCharacter("/mnt/c/test:/path".to_string())));
}

#[test]
fn test_05_home() {
    let cross_path = CrossPathBuf::new(r#"~/test/path"#).unwrap();
    assert_eq!(cross_path.as_str(), "~/test/path");
    assert_eq!(cross_path.to_nix_path_buf().to_string_lossy(), "/home/rustdevuser/test/path");
    assert_eq!(cross_path.to_win_path_buf().to_string_lossy(), "/home/rustdevuser/test/path");
}

/// test join_relative
#[test]
fn test_06_join_relative() {
    let cross_path = CrossPathBuf::new(r#"test/path"#).unwrap();
    let cross_path = cross_path.join_relative("foo/bar").unwrap().join_relative("foo2/bar2").unwrap();
    assert_eq!(cross_path.as_str(), "test/path/foo/bar/foo2/bar2");
    assert_eq!(cross_path.to_nix_path_buf().to_string_lossy(), "test/path/foo/bar/foo2/bar2");
    assert_eq!(cross_path.to_win_path_buf().to_string_lossy(), "test/path/foo/bar/foo2/bar2");
}
