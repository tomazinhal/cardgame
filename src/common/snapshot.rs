use redis::{Commands, RedisError, RedisResult};
use serde::{de::DeserializeOwned, Serialize};

pub trait Snapshot: DeserializeOwned + Serialize {
    fn to_str(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    fn from_str(serialized: &str) -> Self {
        serde_json::from_str(serialized).unwrap()
    }
    fn get_key(&self) -> String;
}

pub trait Cache: Snapshot {
    fn store(&self) -> Result<bool, RedisError> {
        let json = self.to_str();
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let mut con = client.get_connection()?;
        con.set(self.get_key(), json)
    }

    fn load(key: String) -> Self {
        //-> Result<Self, Box<dyn std::error::Error>> {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut con = client.get_connection().unwrap();
        let serialized: Result<String, RedisError> = con.get(key.as_str());

        serde_json::from_str(serialized.unwrap().as_str()).unwrap()
    }
}
