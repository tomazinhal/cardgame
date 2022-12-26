mod common;
use crate::common::models::{Item, ItemList};
use crate::common::snapshot::Snapshot;

impl Snapshot for Item {}
impl Snapshot for ItemList {}

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
