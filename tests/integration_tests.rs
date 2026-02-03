use tomldir::{Config, Value};
use indexmap::IndexMap;
use std::collections::{BTreeMap, HashMap};

#[test]
fn test_basic_load() {
    let toml = r#"
        title = "Test"
        count = 10
        enabled = true
        ratio = 1.5
    "#;
    let cfg = Config::from_str(toml).unwrap();
    assert_eq!(cfg.get_string("title").unwrap(), "Test");
    assert_eq!(cfg.get_int("count").unwrap(), 10);
    assert!(cfg.get_bool("enabled").unwrap());
    assert_eq!(cfg.get_float("ratio").unwrap(), 1.5);
}

#[test]
fn test_nested_table_flattening() {
    let toml = r#"
        [server]
        host = "localhost"
        [server.auth]
        method = "token"
    "#;
    let cfg = Config::from_str(toml).unwrap();
    assert_eq!(cfg.get_string("server.host").unwrap(), "localhost");
    assert_eq!(cfg.get_string("server.auth.method").unwrap(), "token");
}

#[test]
fn test_array_of_tables_flattening() {
    let toml = r#"
        [[users]]
        name = "Alice"
        [[users]]
        name = "Bob"
    "#;
    let cfg = Config::from_str(toml).unwrap();
    assert_eq!(cfg.get_string("users[0].name").unwrap(), "Alice");
    assert_eq!(cfg.get_string("users[1].name").unwrap(), "Bob");
}

#[test]
fn test_primitive_arrays() {
    let toml = r#"
        ports = [80, 443]
    "#;
    let cfg = Config::from_str(toml).unwrap();
    let val = cfg.get("ports").unwrap();
    // With toml::Value, checking type is via type_str()
    assert_eq!(val.type_str(), "array");
    assert!(cfg.get("ports[0]").is_none());
}

#[test]
fn test_flatten_export() {
    let toml = r#"
        [app]
        debug = true
        rate = 5.5
    "#;
    let cfg = Config::from_str(toml).unwrap();
    let flat = cfg.flatten();
    assert_eq!(flat.get("app.debug"), Some(&"true".to_string()));
    assert_eq!(flat.get("app.rate"), Some(&"5.5".to_string()));
}

#[test]
fn test_indexmap_store_ordering() {
    // IndexMap preserves insertion order. 
    let toml = r#"
        z = 1
        a = 2
        c = 3
        b = 4
    "#;
    
    // Explicitly load into IndexMap
    let cfg = Config::from_str_with_store::<IndexMap<String, Value>>(toml).unwrap();
    
    // Verify order
    let keys: Vec<_> = cfg.store().iter().map(|(k, _)| k.as_str()).collect();
    assert_eq!(keys, vec!["z", "a", "c", "b"]);
}

#[test]
fn test_flatten_generic_return() {
    let toml = r#"
        [app]
        id = 1
        name = "test"
    "#;
    let cfg = Config::from_str(toml).unwrap();

    let flat_vec: Vec<(String, String)> = cfg.flatten_into();
    assert_eq!(flat_vec.len(), 2);
    // Note: ordering depends on the underlying store (HashMap)
    assert!(flat_vec.contains(&("app.id".to_string(), "1".to_string())));
    assert!(flat_vec.contains(&("app.name".to_string(), "test".to_string())));
    
    // Flatten to BTreeMap (sorted keys)
    let flat_btree: BTreeMap<String, String> = cfg.flatten_into();
    let keys: Vec<_> = flat_btree.keys().map(|s| s.as_str()).collect();
    assert_eq!(keys, vec!["app.id", "app.name"]);
}

#[test]
fn test_shared_semantics() {
    let toml = "val = 1";
    let cfg = Config::from_str(toml).unwrap();
    let shared = cfg.shared();
    
    assert_eq!(cfg.get_int("val"), Some(1));
    assert_eq!(shared.get_int("val"), Some(1));
}

#[test]
fn test_default_storage_is_hashmap() {
    let toml = "val = 1";
    let cfg = Config::from_str(toml).unwrap();
    assert_eq!(cfg.get_int("val"), Some(1));
}
