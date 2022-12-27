use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    name: String,
    pub id: Uuid,
    timestamp: DateTime<Utc>,
}

impl Item {
    pub fn new(new_name: &str) -> Item {
        Item {
            name: new_name.to_string(),
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
        }
    }

    fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemList {
    name: String,
    list: Vec<Item>,
    pub id: Uuid,
}

impl ItemList {
    pub fn new(list_name: &str) -> ItemList {
        ItemList {
            name: list_name.to_string(),
            id: Uuid::new_v4(),
            list: vec![],
        }
    }

    fn id(&self) -> Uuid {
        self.id
    }

    pub fn add_item(&mut self, item: Item) {
        self.list.push(item);
    }
}
