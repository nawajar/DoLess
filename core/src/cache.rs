use serde::{de::DeserializeOwned, Serialize};

pub trait Cache {
    fn get<T: DeserializeOwned + Clone>(&self, key: &str) -> Option<T>;
    fn set<T: Serialize>(&self, key: &str, value: &T);

    fn set_with<T: Serialize>(&self, key: &str, value: &T, extra: u32) {
        let _ = extra; // ignored by default
        self.set(key, value);
    }
}

#[async_trait::async_trait]
pub trait AsyncCache {
    async fn get<T>(&self, key: &str) -> Option<T>
    where
        T: serde::de::DeserializeOwned + Clone + Send + Sync;

    async fn set<T>(&self, key: &str, value: &T)
    where
        T: serde::Serialize + Send + Sync;

    /// Store with TTL (seconds)
    async fn set_with_ttl<T>(&self, key: &str, value: &T, ttl_secs: u64)
    where
        T: serde::Serialize + Send + Sync;
}
