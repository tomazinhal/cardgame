import pytest
from moders import Foo
from benchmark import Bar, example_rs, example_py


def test_py(benchmark, num_items: int):
    bar = Bar("bar")
    bar.to_json()
    for num in range(num_items):
        bar.add_item(f"item_{num}")
    benchmark(bar.snapshot_store)
    b = Bar.snapshot_load(key=bar.get_id())
    b.to_json() == bar.to_json()
    Bar.from_json(b.to_json())


def test_rs(benchmark, num_items: int):
    foo = Foo("foo")
    foo.to_json()
    for num in range(num_items):
        foo.add_item(f"item_{num}")
    benchmark(foo.snapshot_store)
    f = Foo.snapshot_load(foo.get_id())
    f.to_json() == foo.to_json()
    Foo.from_json(f.to_json())

def test_example_py(benchmark, num_items: int):
    benchmark(example_py, num_items=num_items)

def test_example_rs(benchmark, num_items: int):
    benchmark(example_rs, num_items=num_items)
