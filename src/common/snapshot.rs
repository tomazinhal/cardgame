use serde::{de::DeserializeOwned, Serialize};

pub trait Snapshot: DeserializeOwned + Serialize {
    fn to_str(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    fn from_str(serialized: &str) -> Self {
        serde_json::from_str(serialized).unwrap()
    }
}
