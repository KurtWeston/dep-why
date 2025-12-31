#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Package;
    use std::collections::HashMap;

    fn create_test_lock_file() -> LockFile {
        let mut packages = HashMap::new();
        
        packages.insert(
            "react@18.2.0".to_string(),
            Package {
                name: "react".to_string(),
                version: "18.2.0".to_string(),
                dependencies: HashMap::from([("loose-envify".to_string(), "^1.1.0".to_string())]),
            },
        );
        
        packages.insert(
            "loose-envify@1.4.0".to_string(),
            Package {
                name: "loose-envify".to_string(),
                version: "1.4.0".to_string(),
                dependencies: HashMap::from([("js-tokens".to_string(), "^3.0.0".to_string())]),
            },
        );
        
        packages.insert(
            "js-tokens@3.0.2".to_string(),
            Package {
                name: "js-tokens".to_string(),
                version: "3.0.2".to_string(),
                dependencies: HashMap::new(),
            },
        );
        
        LockFile { packages }
    }

    #[test]
    fn test_trace_finds_package() {
        let lock_file = create_test_lock_file();
        let tracer = DependencyTracer::new(lock_file);
        
        let chains = tracer.trace("js-tokens", None, false).unwrap();
        assert!(!chains.is_empty());
        assert_eq!(chains[0][0].name, "js-tokens");
    }

    #[test]
    fn test_trace_with_version() {
        let lock_file = create_test_lock_file();
        let tracer = DependencyTracer::new(lock_file);
        
        let chains = tracer.trace("js-tokens", Some("3.0.2"), false).unwrap();
        assert!(!chains.is_empty());
        assert_eq!(chains[0][0].version, "3.0.2");
    }

    #[test]
    fn test_trace_nonexistent_package() {
        let lock_file = create_test_lock_file();
        let tracer = DependencyTracer::new(lock_file);
        
        let chains = tracer.trace("nonexistent", None, false).unwrap();
        assert!(chains.is_empty());
    }

    #[test]
    fn test_build_reverse_deps() {
        let lock_file = create_test_lock_file();
        let reverse_deps = DependencyTracer::build_reverse_deps(&lock_file);
        
        assert!(reverse_deps.contains_key("loose-envify"));
        assert!(reverse_deps.contains_key("js-tokens"));
        assert_eq!(reverse_deps["loose-envify"].len(), 1);
    }

    #[test]
    fn test_trace_all_paths() {
        let mut packages = HashMap::new();
        packages.insert(
            "target@1.0.0".to_string(),
            Package {
                name: "target".to_string(),
                version: "1.0.0".to_string(),
                dependencies: HashMap::new(),
            },
        );
        packages.insert(
            "dep-a@1.0.0".to_string(),
            Package {
                name: "dep-a".to_string(),
                version: "1.0.0".to_string(),
                dependencies: HashMap::from([("target".to_string(), "1.0.0".to_string())]),
            },
        );
        packages.insert(
            "dep-b@1.0.0".to_string(),
            Package {
                name: "dep-b".to_string(),
                version: "1.0.0".to_string(),
                dependencies: HashMap::from([("target".to_string(), "1.0.0".to_string())]),
            },
        );
        
        let lock_file = LockFile { packages };
        let tracer = DependencyTracer::new(lock_file);
        
        let chains = tracer.trace("target", None, true).unwrap();
        assert!(chains.len() >= 1);
    }
}