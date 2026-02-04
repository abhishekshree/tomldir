use std::collections::{BTreeMap, HashMap};

#[cfg(feature = "preserve_order")]
use indexmap::IndexMap;
use toml::Value;

/// Default backing store used by `Config`.
///
/// Chosen for:
/// - Fast lookups (O(1) expected)
/// - DoS-resistant hashing (`RandomState`)
/// - Familiar behavior for most users
pub type DefaultStore = HashMap<String, Value>;

/// Internal helper trait for map-like storage.
///
/// This trait is intentionally **minimal**:
/// - No ownership tricks
/// - No iterator boxing
/// - No dynamic dispatch
///
/// It exists solely to let `Config<S>` work with different
/// map implementations *without* leaking complexity into the public API.
pub trait Store: Default + Send + Sync + 'static {
    type Iter<'a>: Iterator<Item = (&'a String, &'a Value)>
    where
        Self: 'a;

    fn insert(&mut self, key: String, value: Value);
    fn get(&self, key: &str) -> Option<&Value>;
    fn iter(&self) -> Self::Iter<'_>;
}

/* ---------------- HashMap ---------------- */

impl<S> Store for HashMap<String, Value, S>
where
    S: std::hash::BuildHasher + Default + Send + Sync + 'static,
{
    type Iter<'a> = std::collections::hash_map::Iter<'a, String, Value>;

    fn insert(&mut self, key: String, value: Value) {
        HashMap::insert(self, key, value);
    }

    fn get(&self, key: &str) -> Option<&Value> {
        HashMap::get(self, key)
    }

    fn iter(&self) -> Self::Iter<'_> {
        HashMap::iter(self)
    }
}

/* ---------------- BTreeMap ---------------- */

impl Store for BTreeMap<String, Value> {
    type Iter<'a> = std::collections::btree_map::Iter<'a, String, Value>;

    fn insert(&mut self, key: String, value: Value) {
        BTreeMap::insert(self, key, value);
    }

    fn get(&self, key: &str) -> Option<&Value> {
        BTreeMap::get(self, key)
    }

    fn iter(&self) -> Self::Iter<'_> {
        BTreeMap::iter(self)
    }
}

/* ---------------- IndexMap ---------------- */

#[cfg(feature = "preserve_order")]
impl Store for IndexMap<String, Value> {
    type Iter<'a> = indexmap::map::Iter<'a, String, Value>;

    fn insert(&mut self, key: String, value: Value) {
        IndexMap::insert(self, key, value);
    }

    fn get(&self, key: &str) -> Option<&Value> {
        IndexMap::get(self, key)
    }

    fn iter(&self) -> Self::Iter<'_> {
        IndexMap::iter(self)
    }
}
