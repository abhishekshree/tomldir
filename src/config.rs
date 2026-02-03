use std::collections::HashMap;
use std::sync::Arc;
use std::path::Path;
use std::fs;
use crate::store::Store;
use crate::error::Result;
use toml::Value;

macro_rules! impl_getters {
    ($( $name:ident => $ret:ty, $method:ident, $doc:literal );* $(;)?) => {
        $(
            #[doc = $doc]
            pub fn $name(&self, key: &str) -> Option<$ret> {
                self.get(key).and_then(Value::$method)
            }
        )*
    };
}

/// Thread-safe configuration container.
///
/// Wraps an `Arc<dyn Store>` to allow cheap cloning and thread transfers.
#[derive(Clone)]
pub struct Config {
    store: Arc<dyn Store>,
}

impl Config {
    /// Loads configuration from a file/path using the default `HashMap` store.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::from_file_with_store::<P, HashMap<String, Value>>(path)
    }

    /// Loads configuration from a TOML string using the default `HashMap` store.
    pub fn from_str(content: &str) -> Result<Self> {
        Self::from_str_with_store::<HashMap<String, Value>>(content)
    }

    /// Loads configuration from a file into a specific Store type.
    ///
    /// Useful if you want ordered keys (via `IndexMap` or `BTreeMap`).
    pub fn from_file_with_store<P, S>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
        S: Store + Default + 'static,
    {
        let content = fs::read_to_string(path)?;
        Self::from_str_with_store::<S>(&content)
    }

    /// Loads configuration from a string into a specific Store type.
    pub fn from_str_with_store<S>(content: &str) -> Result<Self>
    where
        S: Store + Default + 'static,
    {
        let root: Value = toml::from_str(content)?;
        let mut store = S::default();
        flatten_value(&mut store, "", root);
        Ok(Self {
            store: Arc::new(store),
        })
    }

    /// Returns a new instance sharing the same underlying store.
    ///
    /// This is an explicit, cheap clone of the internal `Arc`.
    pub fn shared(&self) -> Self {
        Self {
            store: Arc::clone(&self.store),
        }
    }

    /// Helper to get a reference to the inner store.
    pub fn store(&self) -> &dyn Store {
        &*self.store
    }

    /// Generic retrieval of a Value by key.
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.store.get(key)
    }

    impl_getters! {
        get_string => &str, as_str, "Helper to get a String value.";
        get_int => i64, as_integer, "Helper to get an integer value.";
        get_float => f64, as_float, "Helper to get a float value.";
        get_bool => bool, as_bool, "Helper to get a boolean value.";
    }

    /// Returns a flattened copy of the configuration as a `HashMap<String, String>`.
    ///
    /// All values are converted to strings.
    pub fn flatten(&self) -> HashMap<String, String> {
        self.flatten_into()
    }

    /// Returns a flattened collection of the configuration, where all values are converted to strings.
    ///
    /// The return type `C` must implement `FromIterator<(String, String)>`.
    /// This allows you to collect into `HashMap`, `BTreeMap`, `IndexMap`, or `Vec`.
    ///
    /// # Example
    /// ```rust
    /// use std::collections::BTreeMap;
    /// # use tomldir::Config;
    /// # let cfg = Config::from_str("key = 'val'").unwrap();
    /// let map: BTreeMap<String, String> = cfg.flatten_into();
    /// ```
    pub fn flatten_into<C>(&self) -> C
    where
        C: FromIterator<(String, String)>,
    {
        self.store
            .iter()
            .map(|(k, v)| {
                // toml::Value defaults to double quoting strings in to_string() (JSON style).
                // If it's a string, we want the raw string content for "flattening".
                // Otherwise use the default Display repr.
                let s = if let Some(str_val) = v.as_str() {
                    str_val.to_string()
                } else {
                    v.to_string()
                };
                (k.clone(), s)
            })
            .collect()
    }
}

/// Recursive helper to flatten the TOML tree into the store.
fn flatten_value<S: Store + ?Sized>(store: &mut S, prefix: &str, value: Value) {
    match value {
        Value::Table(t) => {
            for (k, v) in t {
                let new_key = if prefix.is_empty() {
                    k
                } else {
                    format!("{}.{}", prefix, k)
                };
                flatten_value(store, &new_key, v);
            }
        }
        Value::Array(a) => {
            // Check if array contains tables. If so, flatten with indices.
            let is_table_array = a.first().map_or(false, |v| v.is_table());
            
            if is_table_array {
                 for (i, v) in a.into_iter().enumerate() {
                    let new_key = format!("{}[{}]", prefix, i);
                    flatten_value(store, &new_key, v);
                }
            } else {
                // Primitive array, keep as Value::Array
                if !prefix.is_empty() {
                    store.insert(prefix.to_string(), Value::Array(a));
                }
            }
        }
        _ => {
            // Primitive (String, Int, Float, Bool, Datetime)
            if !prefix.is_empty() {
                store.insert(prefix.to_string(), value);
            }
        }
    }
}
