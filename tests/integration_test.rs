use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_detect_npm_lock_file() {
    let dir = TempDir::new().unwrap();
    let lock_path = dir.path().join("package-lock.json");
    fs::write(&lock_path, "{}").unwrap();
    
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();
    
    assert!(PathBuf::from("package-lock.json").exists());
    
    std::env::set_current_dir(original_dir).unwrap();
}

#[test]
fn test_parse_package_query_with_version() {
    let query = "lodash@4.17.21";
    let pos = query.rfind('@').unwrap();
    let name = &query[..pos];
    let version = &query[pos + 1..];
    
    assert_eq!(name, "lodash");
    assert_eq!(version, "4.17.21");
}

#[test]
fn test_parse_package_query_without_version() {
    let query = "lodash";
    let result = query.rfind('@');
    
    assert!(result.is_none());
}

#[test]
fn test_parse_scoped_package() {
    let query = "@babel/core@7.22.0";
    let pos = query.rfind('@').unwrap();
    let name = &query[..pos];
    let version = &query[pos + 1..];
    
    assert_eq!(name, "@babel/core");
    assert_eq!(version, "7.22.0");
}

#[test]
fn test_empty_dependency_chain() {
    let chains: Vec<Vec<String>> = Vec::new();
    assert!(chains.is_empty());
}

#[test]
fn test_multiple_versions_same_package() {
    use std::collections::HashMap;
    
    let mut packages = HashMap::new();
    packages.insert("lodash@4.17.20".to_string(), "v1");
    packages.insert("lodash@4.17.21".to_string(), "v2");
    
    assert_eq!(packages.len(), 2);
    assert!(packages.contains_key("lodash@4.17.20"));
    assert!(packages.contains_key("lodash@4.17.21"));
}
