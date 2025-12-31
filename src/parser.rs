#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_file(dir: &TempDir, name: &str, content: &str) -> PathBuf {
        let path = dir.path().join(name);
        fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn test_parse_npm_v2_format() {
        let dir = TempDir::new().unwrap();
        let content = r#"{
  "name": "test",
  "lockfileVersion": 2,
  "dependencies": {
    "lodash": {
      "version": "4.17.21",
      "resolved": "https://registry.npmjs.org/lodash/-/lodash-4.17.21.tgz"
    }
  },
  "packages": {
    "": {
      "dependencies": {
        "lodash": "^4.17.21"
      }
    },
    "node_modules/lodash": {
      "version": "4.17.21"
    }
  }
}"#;
        let path = create_test_file(&dir, "package-lock.json", content);
        let lock_file = LockFile::parse(&path).unwrap();
        
        assert!(lock_file.packages.contains_key("lodash@4.17.21"));
        let pkg = &lock_file.packages["lodash@4.17.21"];
        assert_eq!(pkg.name, "lodash");
        assert_eq!(pkg.version, "4.17.21");
    }

    #[test]
    fn test_parse_yarn_lock() {
        let dir = TempDir::new().unwrap();
        let content = r#"# yarn lockfile v1

lodash@^4.17.21:
  version "4.17.21"
  resolved "https://registry.yarnpkg.com/lodash/-/lodash-4.17.21.tgz"

react@^18.0.0:
  version "18.2.0"
  resolved "https://registry.yarnpkg.com/react/-/react-18.2.0.tgz"
  dependencies:
    loose-envify "^1.1.0"
"#;
        let path = create_test_file(&dir, "yarn.lock", content);
        let lock_file = LockFile::parse(&path).unwrap();
        
        assert!(lock_file.packages.contains_key("lodash@4.17.21"));
        assert!(lock_file.packages.contains_key("react@18.2.0"));
        let react = &lock_file.packages["react@18.2.0"];
        assert_eq!(react.dependencies.len(), 1);
        assert!(react.dependencies.contains_key("loose-envify"));
    }

    #[test]
    fn test_parse_pnpm_lock() {
        let dir = TempDir::new().unwrap();
        let content = r#"lockfileVersion: '6.0'

dependencies:
  lodash:
    specifier: ^4.17.21
    version: 4.17.21

packages:
  /lodash@4.17.21:
    resolution: {integrity: sha512-test}
    dev: false
"#;
        let path = create_test_file(&dir, "pnpm-lock.yaml", content);
        let lock_file = LockFile::parse(&path).unwrap();
        
        assert!(lock_file.packages.contains_key("lodash@4.17.21"));
    }

    #[test]
    fn test_parse_nonexistent_file() {
        let result = LockFile::parse(Path::new("nonexistent.json"));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_unsupported_format() {
        let dir = TempDir::new().unwrap();
        let path = create_test_file(&dir, "unknown.lock", "{}");
        let result = LockFile::parse(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_json() {
        let dir = TempDir::new().unwrap();
        let path = create_test_file(&dir, "package-lock.json", "invalid json");
        let result = LockFile::parse(&path);
        assert!(result.is_err());
    }
}