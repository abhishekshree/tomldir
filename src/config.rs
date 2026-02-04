use std::{fs, path::Path, sync::Arc};

use toml::Value;

use crate::{
    error::Result,
    store::{DefaultStore, Store},
};

/// Thread-safe, immutable configuration container.
///
/// ## Design notes
///
/// - Configuration is **parsed once** and then treated as read-only.
/// - Internally, values are **flattened** into dot-separated keys:
///   `database.host`, `runners[0].name`, etc.
/// - The backing store is wrapped in an `Arc` to make cloning cheap and sharing
///   across threads trivial.
///
/// ```rust
/// use tomldir::Config;
/// # use std::fs;
/// # let _ = fs::write("config.toml", "title = 'Test'");
/// let cfg = Config::from_file("config.toml").unwrap();
/// # fs::remove_file("config.toml").unwrap();
/// ```
///
/// You can also use a custom store like:
///
/// ```rust
/// use std::collections::HashMap;
///
/// use rustc_hash::FxBuildHasher;
/// use tomldir::{Config, Value};
///
/// let cfg = Config::<HashMap<String, Value, FxBuildHasher>>::from_file_with("config.toml");
/// ```
pub struct Config<S = DefaultStore> {
    store: Arc<S>,
}

impl<S> Clone for Config<S> {
    fn clone(&self) -> Self {
        Self {
            store: Arc::clone(&self.store),
        }
    }
}

impl Config {
    /// Load configuration from a TOML file using the default store.
    ///
    /// # Errors
    /// Returns an error if the file cannot be read or contains invalid TOML.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::from_file_with(path)
    }

    /// Load configuration from a TOML string using the default store.
    ///
    /// # Errors
    /// Returns an error if the string contains invalid TOML.
    pub fn from_toml(content: &str) -> Result<Self> {
        Self::from_toml_with(content)
    }
}

// This is me having fun with macros
macro_rules! impl_getters {
    ($( $name:ident => $ret:ty, $method:ident, $doc:literal );* $(;)?) => {
        $(
            #[doc = $doc]
            #[must_use]
            pub fn $name(&self, key: &str) -> Option<$ret> {
                self.get(key).and_then(Value::$method)
            }
        )*
    };
}

impl<S> Config<S>
where
    S: Store,
{
    /// Load configuration from a TOML file using a custom store.
    ///
    /// # Errors
    /// Returns an error if the file cannot be read or contains invalid TOML.
    pub fn from_file_with<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Self::from_toml_with(&content)
    }

    /// Load configuration from a TOML string using a custom store.
    ///
    /// # Errors
    /// Returns an error if the string contains invalid TOML.
    pub fn from_toml_with(content: &str) -> Result<Self> {
        let root: Value = toml::from_str(content)?;

        let mut store = S::default();
        flatten_value(&mut store, "", root);

        Ok(Self {
            store: Arc::new(store),
        })
    }

    /// Returns a new instance sharing the same underlying store.
    #[must_use]
    pub fn shared(&self) -> Self {
        self.clone()
    }

    /// Retrieve a raw TOML value by flattened key.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.store.get(key)
    }

    impl_getters! {
        get_string => &str, as_str, "Helper to get a String value.";
        get_int => i64, as_integer, "Helper to get an integer value.";
        get_float => f64, as_float, "Helper to get a float value.";
        get_bool => bool, as_bool, "Helper to get a boolean value.";
    }

    /// Flatten all values into string form.
    ///
    /// Strings preserve their raw content.
    /// Non-strings use TOML's display representation.
    pub fn flatten(&self) -> impl Iterator<Item = (String, String)> + '_ {
        self.store.iter().map(|(k, v)| {
            let value = v
                .as_str()
                .map_or_else(|| v.to_string(), ToString::to_string);
            (k.clone(), value)
        })
    }

    /// Returns a flattened collection of the configuration, where all values
    /// are converted to strings.
    ///
    /// The return type `C` must implement `FromIterator<(String, String)>`.
    #[must_use]
    pub fn flatten_into<C>(&self) -> C
    where
        C: FromIterator<(String, String)>,
    {
        self.flatten().collect()
    }
}

/// Recursively flatten a TOML value into dot-separated keys like:
///
/// - `{ a = { b = 1 } }` → `a.b = 1`
/// - `{ a = [ { x = 1 } ] }` → `a[0].x = 1`
fn flatten_value<S: Store>(store: &mut S, prefix: &str, value: Value) {
    match value {
        Value::Table(table) => {
            for (k, v) in table {
                let key = if prefix.is_empty() {
                    k
                } else {
                    format!("{prefix}.{k}")
                };
                flatten_value(store, &key, v);
            }
        }

        Value::Array(array) => {
            let is_table_array = array.first().is_some_and(Value::is_table);

            if is_table_array {
                for (i, v) in array.into_iter().enumerate() {
                    let key = format!("{prefix}[{i}]");
                    flatten_value(store, &key, v);
                }
            } else if !prefix.is_empty() {
                store.insert(prefix.to_string(), Value::Array(array));
            }
        }

        other => {
            if !prefix.is_empty() {
                store.insert(prefix.to_string(), other);
            }
        }
    }
}
