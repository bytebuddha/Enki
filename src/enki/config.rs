use serde::de::DeserializeOwned;
use serde_json::from_value;
use serde_json::Value;

use std::collections::HashMap;

/// The configuration used to store variables for the editor.
/// The internal representation is a `HashMap<String, Value>`
/// where `Value` is a `serde_json::Value`. Variables can be
/// inserted with the `set` method and retrieived with either
/// the `get`, 'get_default` or `get_default_t` methods.
#[derive(Default)]
pub struct Configuration {
    data: HashMap<String, Value>,
}

impl Configuration {
    pub fn from(data: HashMap<String, Value>) -> Configuration {
        Configuration { data }
    }

    /// Set a configuration variable. Will remove the current value
    /// if there is any.
    pub fn set<S: Into<String>>(&mut self, key: S, value: Value) {
        self.data.insert(key.into(), value);
    }

    pub fn vars(&self) -> Vec<(&String, &Value)> {
        self.data.iter().collect()
    }

    pub fn unset<S: AsRef<str>>(&mut self, key: S) {
        self.data.remove(key.as_ref());
    }

    /// Get the value of a configuration variable.
    pub fn get<S: Into<String>>(&mut self, key: S) -> Option<Value> {
        self.data.get(&key.into()).map(Clone::clone)
    }

    pub fn get_t<S: Into<String>, T: DeserializeOwned>(&self, key: S) -> Option<T> {
        if let Some(value) = self.data.get(&key.into()).map(Clone::clone) {
            if let Ok(value) = from_value(value) {
                return Some(value);
            }
        }
        None
    }

    /// Get the value of a configuration key. returns `value` if None was found.
    pub fn get_default<S: Into<String>>(&self, key: S, value: Value) -> Value {
        self.data
            .get(&key.into())
            .map(Clone::clone)
            .unwrap_or(value)
    }

    /// Get the value of a configuration key deserialized using `serde_json::from_value`.
    /// Returns `value` if not set or deserializion failed.
    pub fn get_default_t<S: Into<String>, T: DeserializeOwned>(&self, key: S, value: T) -> T {
        if let Some(value) = self.data.get(&key.into()).map(Clone::clone) {
            if let Ok(value) = from_value(value) {
                return value;
            }
        }
        value
    }
}
