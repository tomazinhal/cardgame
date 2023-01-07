import asyncio
from uuid import uuid4, UUID
from datetime import datetime
from moders import Foo
from aioredis import Redis
import aioredis
import related as rl


async def get_redis():
    conn = await aioredis.create_connection("redis://localhost/0")
    return conn


async def save_to_redis(key, value):
    conn = await get_redis()
    i = rl.to_json(value)
    await conn.execute("set", key, i)


async def get_from_redis(key):
    conn = await get_redis()
    state = await conn.execute("get", key)
    return rl.from_json(state)


@rl.mutable
class Item(object):
    name = rl.StringField()
    id = rl.StringField(required=False, default=uuid4())
    timestamp = rl.DateTimeField(required=False, default=datetime.utcnow())


@rl.mutable
class ItemList(object):
    name = rl.StringField()
    id = rl.StringField(required=False, default=uuid4())
    itemlist = rl.SequenceField(Item, required=False)

    def add_item(self, item: Item):
        self.items.append(item)


class Bar:
    def __init__(self, name: str):
        self.itemlist = ItemList(name=name)

    def get_id(self):
        return str(self.itemlist.id)

    def to_json(self):
        d = rl.to_dict(self.itemlist)
        return rl.to_json(d)

    def snapshot_store(self):
        asyncio.get_event_loop().run_until_complete(
            save_to_redis(self.itemlist.id, self.itemlist)
        )

    @classmethod
    def snapshot_load(cls, key: str):
        state = asyncio.get_event_loop().run_until_complete(get_from_redis(key))
        return rl.to_model(cls, state)

    @classmethod
    def from_json(cls, serialized: str):
        return rl.from_json(serialized)


def py():
    bar = Bar("bar")
    bar.to_json()
    bar.snapshot_store()
    b = Bar.snapshot_load(key=bar.get_id())
    b.to_json() == bar.to_json()
    Bar.from_json(b.to_json())


def rs():
    foo = Foo("foo")
    foo.to_json()
    foo.snapshot_store()
    f = Foo.snapshot_load(foo.get_id())
    f.to_json() == foo.to_json()
    Foo.from_json(f.to_json())


if __name__ == "__main__":
    py()
    rs()
