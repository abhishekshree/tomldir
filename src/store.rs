use std::collections::{BTreeMap, HashMap};

use indexmap::IndexMap;
use toml::Value;

/// Trait for the underlying configuration storage.
///
/// Allows swapping between `HashMap`, `BTreeMap`, `IndexMap`, or custom
/// implementations.
pub trait Store: Send + Sync {
    /// Inserts a key-value pair into the store.
    fn insert(&mut self, key: String, value: Value);

    /// Retrieves a value by key.
    fn get(&self, key: &str) -> Option<&Value>;

    /// Returns an iterator over the store's entries.
    fn iter(&self) -> Box<dyn Iterator<Item = (&String, &Value)> + '_>;
}

impl<S: std::hash::BuildHasher + Send + Sync> Store for HashMap<String, Value, S> {
    fn insert(&mut self, key: String, value: Value) {
        self.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&Value> {
        self.get(key)
    }

    fn iter(&self) -> Box<dyn Iterator<Item = (&String, &Value)> + '_> {
        Box::new(self.iter())
    }
}

impl Store for BTreeMap<String, Value> {
    fn insert(&mut self, key: String, value: Value) {
        self.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&Value> {
        self.get(key)
    }

    fn iter(&self) -> Box<dyn Iterator<Item = (&String, &Value)> + '_> {
        Box::new(self.iter())
    }
}

impl Store for IndexMap<String, Value> {
    fn insert(&mut self, key: String, value: Value) {
        self.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&Value> {
        self.get(key)
    }

    fn iter(&self) -> Box<dyn Iterator<Item = (&String, &Value)> + '_> {
        Box::new(self.iter())
    }
}
