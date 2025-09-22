use super::*;

#[test]
fn test_01() {
    let cross_path = CrossPathBuf::new("test/path".to_string());
    assert_eq!(cross_path.as_str(), "test/path");
    assert_eq!(cross_path.to_nix_path_buf().to_string_lossy(), "test/path");
    assert_eq!(cross_path.to_win_path_buf().to_string_lossy(), "test/path");
}

#[test]
fn test_02() {
    let cross_path = CrossPathBuf::new(r#"test\path"#.to_string());
    assert_eq!(cross_path.as_str(), "test/path");
    assert_eq!(cross_path.to_nix_path_buf().to_string_lossy(), "test/path");
    assert_eq!(cross_path.to_win_path_buf().to_string_lossy(), "test/path");
}

#[test]
fn test_03() {
    let cross_path = CrossPathBuf::new(r#"c:\test\path"#.to_string());
    assert_eq!(cross_path.as_str(), "/mnt/c/test/path");
    assert_eq!(cross_path.to_nix_path_buf().to_string_lossy(), "/mnt/c/test/path");
    assert_eq!(cross_path.to_win_path_buf().to_string_lossy(), "c:/test/path");
}

#[test]
fn test_03a() {
    let cross_path = CrossPathBuf::new(r#"c:/test/path"#.to_string());
    assert_eq!(cross_path.as_str(), "/mnt/c/test/path");
    assert_eq!(cross_path.to_nix_path_buf().to_string_lossy(), "/mnt/c/test/path");
    assert_eq!(cross_path.to_win_path_buf().to_string_lossy(), "c:/test/path");
}

#[test]
fn test_04() {
    let cross_path = CrossPathBuf::new(r#"c:\test:\path"#.to_string());
    assert_eq!(cross_path.as_str(), "/mnt/c/test/path");
    assert_eq!(cross_path.to_nix_path_buf().to_string_lossy(), "/mnt/c/test/path");
    assert_eq!(cross_path.to_win_path_buf().to_string_lossy(), "c:/test/path");
}

#[test]
fn test_05() {
    let cross_path = CrossPathBuf::new(r#"~/test/path"#.to_string());
    assert_eq!(cross_path.as_str(), "~/test/path");
    assert_eq!(cross_path.to_nix_path_buf().to_string_lossy(), "/home/rustdevuser/test/path");
    assert_eq!(cross_path.to_win_path_buf().to_string_lossy(), "/home/rustdevuser/test/path");
}
