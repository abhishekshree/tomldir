# tomldir

[![Crates.io](https://img.shields.io/crates/v/tomldir.svg)](https://crates.io/crates/tomldir)
[![Docs](https://docs.rs/tomldir/badge.svg)](https://docs.rs/tomldir)
[![CI](https://github.com/abhishekshree/tomldir/actions/workflows/ci.yml/badge.svg)](https://github.com/abhishekshree/tomldir/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**tomldir** is a small, opinionated Rust crate for loading TOML configuration files into **map-based data structures**, optimized for runtime access, flattening, and configuration composition.

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
tomldir = "0.1"
# Optional: for order-preserving keys
indexmap = "2.0"
```

### Basic Usage

```rust
use tomldir::Config;

fn main() -> tomldir::Result<()> {
    let toml_data = r#"
        [database]
        host = "localhost"
        port = 5432
    "#;

    // Load into default HashMap
    let cfg = Config::from_str(toml_data)?;
    
    // Thread-safe access (explicit cheap clone)
    let cfg = cfg.shared(); 

    assert_eq!(cfg.get_string("database.host"), Some("localhost"));
    Ok(())
}
```

### Advanced: Preserving Order with `IndexMap`

If you need to iterate over keys in the order they were defined (e.g., for help text generation or consistent output), use `IndexMap`.

```rust
use tomldir::{Config, Value};
use indexmap::IndexMap;

fn main() -> tomldir::Result<()> {
    let toml_data = r#"
        first = 1
        second = 2
    "#;

    // Specify storage type explicitly
    let cfg = Config::from_str_with_store::<IndexMap<String, Value>>(toml_data)?;

    let keys: Vec<_> = cfg.store().iter().map(|(k, _)| k.as_str()).collect();
    assert_eq!(keys, vec!["first", "second"]);
    
    Ok(())
}
```

### Flattening

You can flatten the config into a simple `HashMap<String, String>`:

```rust
let flat = cfg.flatten();
```

Or flatten into any custom type using `flatten_into()`:

```rust
use std::collections::BTreeMap;

// Flatten to BTreeMap (sorted keys)
let flat_sorted: BTreeMap<String, String> = cfg.flatten_into();

// Flatten to Vec
let flat_vec: Vec<(String, String)> = cfg.flatten_into();
```

## Features

* **TOML-only**: Built directly on `toml::Value`.
* **Map-first**: Stores data as `HashMap` by default, but swapable for `IndexMap` or `BTreeMap`.
* **Thread-safe**: Designed for `Arc` usage and concurrency.
* **Deterministic Flattening**: Nested tables become dot-separated keys.

## License

MIT
