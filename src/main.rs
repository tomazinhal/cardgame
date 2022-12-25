use uuid::Uuid;
use serde::{de::DeserializeOwned, Serialize, Deserialize};

trait Snapshot: DeserializeOwned + Serialize {
    fn to_str(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    fn from_str(serialized: &str) -> Self {
        serde_json::from_str(serialized).unwrap()
    }
}


#[derive(Debug, Serialize, Deserialize)]
struct Item {
    name: String,
    id: Uuid,
}

impl Item {
    fn new(new_name: &str) -> Item {
        Item {
            name: new_name.to_string(),
            id: Uuid::new_v4(),
        }
    }
}

impl Snapshot for Item { }

#[derive(Debug, Serialize, Deserialize)]
struct ItemList {
    name: String,
    list: Vec<Item>,
    id: Uuid,

}

impl ItemList {
    fn new(list_name: &str) -> ItemList {
        ItemList {
            name: list_name.to_string(),
            id: Uuid::new_v4(),
            list: vec![],
        }
    }

    fn add_item(&mut self, item: Item) {
        self.list.push(item);
    }
}

impl Snapshot for ItemList { }


fn main() {
    println!("Hello, world!");
    let item: Item = Item::new("foo");
    println!("{:?}", item);
    let serialized = item.to_str();
    println!("{:?}", serialized);
    let deserialized: Item = Item::from_str(serialized.as_str());
    println!("{:?}", deserialized);
    let mut itemlist = ItemList::new("basket");
    itemlist.add_item(item);
    let serialized = itemlist.to_str();
    println!("Serialized list: {:?}", serialized);
    let de_list: ItemList = ItemList::from_str(serialized.as_str());
    println!("{:?}", de_list);
}


