mod common;
use crate::common::models::{Item, ItemList};
use crate::common::snapshot::{Cache, Snapshot};

impl Snapshot for Item {
    fn get_key(&self) -> String {
        self.id.to_string()
    }
}
impl Snapshot for ItemList {
    fn get_key(&self) -> String {
        self.id.to_string()
    }
}
impl Cache for Item {}
impl Cache for ItemList {}

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
    let it_worked = de_list.store();
    ItemList::load(de_list.get_key());
}
