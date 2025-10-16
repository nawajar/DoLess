

# DoLess 🦀 — Procedural Macros for Data Mapping and Caching

`DoLess` is a Rust library offering **procedural macros** that simplify both
**data-to-struct mapping** and **cache integration** patterns.

It provides two main features:

- 🧩 **`#[derive(FromHashMap)]`** — auto-generates a type-safe
  `From<HashMap<String, String>>` implementation for simple and nested structs.
- ⚡ **`#[cache_it(...)]`** — injects cache lookup logic directly into your functions,  
  now supporting **both sync and async** functions.

---

## 🚀 Features

### 🧩 Mapping Features
- ✅ Auto-maps from `HashMap<String, String>`
- 🔢 Supports types: `String`, numeric primitives, `bool`, `Option<T>`
- ➕ Supports lists: `Vec<T>`, `Vec<Option<T>>`
- 🪆 Nested structs with dot notation (`details.name`)
- ⚙ Defaults for missing fields

### ⚡ Cache Macro Features
- 📦 Add `#[cache_it(...)]` to perform cache lookups automatically
- 🗝 Configurable options:
  - `key = "some:key"`
  - `key = format!("user:{}", id)`
  - `var = redis` — custom cache variable name
  - `name = cached_data` — custom binding name
- 🔄 Works with any cache backend implementing `Cache` or `AsyncCache`
- ⚙ Async-aware — automatically inserts `.await` where needed

---

## 📦 Installation

```toml
[dependencies]
doless = "0.4.1"
```

Includes:
- `doless_core` — Cache and AsyncCache traits
- `doless_macros` — Procedural macros
- `doless` — Unified re-export crate

---

## ✨ Usage Examples

### 🧩 Example 1 — Struct Mapping with `FromHashMap`

```rust
use doless::FromHashMap;
use std::collections::HashMap;

#[derive(FromHashMap, Debug)]
struct Car {
    model: String,
    brand: String,
    details: CarDetails,
    tags: Vec<String>,
}

#[derive(FromHashMap, Debug)]
struct CarDetails {
    name: String,
    description: String,
}

fn main() {
    let mut data = HashMap::new();
    data.insert("model".into(), "GT-R".into());
    data.insert("brand".into(), "Nissan".into());
    data.insert("details.name".into(), "Skyline".into());
    data.insert("details.description".into(), "Legendary Sports Car".into());
    data.insert("tags".into(), "fast,collectible,cool".into());

    let car: Car = Car::from(data);
    println!("{:#?}", car);
}
```

---

### ⚡ Example 2 — Caching (Sync)

```rust
use doless::cache_it;
use doless_core::cache::Cache;
use serde::{Serialize, de::DeserializeOwned};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Clone, Default)]
struct DummyCache {
    store: Arc<Mutex<HashMap<String, String>>>,
}

impl Cache for DummyCache {
    fn get<T: DeserializeOwned + Clone>(&self, key: &str) -> Option<T> {
        let guard = self.store.lock().ok()?;
        serde_json::from_str(guard.get(key)?).ok()
    }

    fn set<T: Serialize>(&self, key: &str, value: &T) {
        if let Ok(json) = serde_json::to_string(value) {
            if let Ok(mut m) = self.store.lock() {
                m.insert(key.into(), json);
            }
        }
    }
}

#[cache_it(key = "user:list")]
fn get_users(cache: &impl Cache) -> Vec<String> {
    let cache_data: Option<Vec<String>> = cache_data;
    if let Some(users) = cache_data {
        return users;
    }
    let users = vec!["alice".into(), "bob".into()];
    cache.set("user:list", &users);
    users
}

fn main() {
    let cache = DummyCache::new();
    println!("{:?}", get_users(&cache));
}
```

---

### ⚙ Example 3 — Async Caching with `AsyncCache`

```rust
use doless::cache_it;
use doless_core::cache::AsyncCache;
use serde::{Serialize, de::DeserializeOwned};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Clone, Default)]
struct RedisCache {
    store: Arc<Mutex<HashMap<String, String>>>,
}

#[async_trait::async_trait]
impl AsyncCache for RedisCache {
    async fn get<T>(&self, key: &str) -> Option<T>
    where
        T: DeserializeOwned + Clone + Send + Sync,
    {
        let guard = self.store.lock().ok()?;
        serde_json::from_str(guard.get(key)?).ok()
    }

    async fn set<T>(&self, key: &str, value: &T)
    where
        T: Serialize + Send + Sync,
    {
        if let Ok(json) = serde_json::to_string(value) {
            if let Ok(mut map) = self.store.lock() {
                map.insert(key.to_string(), json);
            }
        }
    }

    async fn set_with_ttl<T>(&self, key: &str, value: &T, ttl_secs: u64)
    where
        T: Serialize + Send + Sync,
    {
        println!("Setting with TTL: {} seconds", ttl_secs);
        self.set(key, value).await;
    }
}

#[cache_it(key = "user:async")]
async fn get_user_async(cache: &impl AsyncCache) -> Option<String> {
    let cached: Option<String> = cache_data;
    if cached.is_some() {
        return cached;
    }

    let data = String::from("jeff");
    cache.set("user:async", &data).await;
    Some(data)
}

#[tokio::main]
async fn main() {
    let cache = RedisCache::new();
    let user = get_user_async(&cache).await;
    println!("User = {:?}", user);
}
```

🧠 The macro detects `async fn` automatically and inserts `.await` where needed.

---

## 🧭 Roadmap

| Feature                               |  Status   |
| ------------------------------------- | :-------: |
| `FromHashMap` with nested struct      |     ✅     |
| `Vec<T>` and `Vec<Option<T>>` support |     ✅     |
| Synchronous cache macro               |     ✅     |
| **Async cache support**               |   ✅ NEW   |
| TTL + extended cache (via `set_with`) |     ✅     |
| Error diagnostics and reporting       | 🚧 Planned |


