## To snapshot a model

I want to create models in rust and write them to Redis.

An Item is a model that contains an ID, name and datetime.
I want to be able to serialize this model to JSON and store it to redis.
I want to get a model from Redis and deserialize it into a model.


```rust
fn main() {
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
    // it's the same as itemlist :D
}
```


Apparently working :)
```sh
❯ redis-cli
127.0.0.1:6379> keys *
1) "69433fe1-8faf-4975-8219-bf6806ebc8c2"
2) "543e8a42-d622-4510-a634-6e26637388d2"
3) "bef9fb13-230c-4f8f-bb67-1e5019147187"
4) "foo"
```

## TODO

[ ] Use Pyo3 bindings to make this a module available in Python.
