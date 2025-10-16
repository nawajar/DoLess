use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use doless::cache_it;
use doless_core::cache::Cache;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct User {
    id: u32,
    name: String,
    maybe: Option<String>,
}

#[derive(Clone, Default)]
struct DummyCache {
    store: Arc<Mutex<HashMap<String, String>>>,
}

impl DummyCache {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Cache for DummyCache {
    fn get<T: DeserializeOwned + Clone>(&self, key: &str) -> Option<T> {
        let guard = self.store.lock().ok()?;
        serde_json::from_str(guard.get(key)?).ok()
    }

    fn set<T: Serialize>(&self, key: &str, value: &T) {
        if let Ok(json) = serde_json::to_string(value) {
            if let Ok(mut map) = self.store.lock() {
                map.insert(key.to_string(), json);
            }
        }
    }

    fn set_with<T: Serialize>(&self, key: &str, value: &T, extra: u32) {
        //May set extra u32 with cache ttl.
        self.set(key, value);
    }
}

//
// -- Macro-driven test functions
//

/// 🧩 Default usage — static key, Vec<String> type
#[cache_it(key = "user:list")]
fn get_names(cache: &impl Cache) -> Vec<String> {
    let infer_user: Option<Vec<String>> = cache_data;
    if let Some(mut users) = infer_user {
        users.sort();

        return users;
    }

    let result = vec!["alice".into(), "bob".into()];
    cache.set("user:list", &result);
    //cache.set_with("user:list", &result, 32);

    result
}

/// 🧩 Static key, struct type
#[cache_it(key = "user")]
fn get_user(cache: &impl Cache) -> Option<User> {
    cache_data
}

/// 🧩 Custom cache variable name and binding name
#[cache_it(var = redis, key = "user_custom", name = cached_user)]
fn get_user_custom_var(redis: &impl Cache) -> Option<User> {
    cached_user
}

/// 🧩 Runtime dynamic key expression
#[cache_it(key = format!("user:{}", id))]
fn get_user_custom_dynamic_key(id: u32, cache: &impl Cache) -> Option<User> {
    cache_data
}

//
// -- Helper functions
//

fn default_user(id: u32, name: &str, maybe: Option<&str>) -> User {
    User {
        id,
        name: name.to_string(),
        maybe: maybe.map(|s| s.to_string()),
    }
}

//
// -- Tests grouped by behavior
//

#[test]
fn test_list_cache_miss_populates_store() {
    let cache = DummyCache::new();
    let users = get_names(&cache);
    assert_eq!(users, vec!["alice".to_string(), "bob".to_string()]);
}

#[test]
fn test_list_cache_hit_returns_existing() {
    let cache = DummyCache::new();
    let items = vec![String::from("john"), String::from("snow")];
    cache.set("user:list", &items);

    let users = get_names(&cache);
    assert_eq!(users, items, "should return existing cached names");
}

#[test]
fn test_user_cache_miss_returns_none() {
    let cache = DummyCache::new();
    assert!(
        get_user(&cache).is_none(),
        "should not find user when cache empty"
    );
}

#[test]
fn test_user_cache_hit_returns_data() {
    let cache = DummyCache::new();
    let user = default_user(1, "jeff", Some("jeffy"));
    cache.set("user", &user);

    let cached = get_user(&cache).expect("expected cached user");
    assert_eq!(cached, user);
}

#[test]
fn test_custom_var_cache_hit() {
    let cache = DummyCache::new();
    let user = default_user(1, "jane", None);
    cache.set("user_custom", &user);

    let cached = get_user_custom_var(&cache).expect("expected cached user");
    assert_eq!(cached, user);
}

#[test]
fn test_dynamic_key_cache_hit() {
    let cache = DummyCache::new();
    let user = default_user(2, "peter", None);
    let key = format!("user:{}", user.id);
    cache.set(&key, &user);

    let cached = get_user_custom_dynamic_key(user.id, &cache).expect("expected cached user");
    assert_eq!(cached, user);
}

#[test]
fn test_dynamic_key_miss_returns_none() {
    let cache = DummyCache::new();
    assert!(get_user_custom_dynamic_key(10, &cache).is_none());
}
