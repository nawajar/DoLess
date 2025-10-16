use serde::{de::DeserializeOwned, Serialize};

pub trait Cache {
    fn get<T: DeserializeOwned + Clone>(&self, key: &str) -> Option<T>;
    fn set<T: Serialize>(&self, key: &str, value: &T);

    fn set_with<T: Serialize>(&self, key: &str, value: &T, extra: u32) {
        let _ = extra; // ignored by default
        self.set(key, value);
    }
}
