## To snapshot a model

I want to create models in rust and write them to Redis.

An Item is a model that contains an ID, name and datetime.
I want to be able to serialize this model to JSON and store it to redis.
I want to get a model from Redis and deserialize it into a model.

Check [results.md](Result) for benchmark results.


```py
from moders import Foo
foo = Foo("foo")
foo.to_json()
foo.snapshot_store()
# get the id

f = Foo.snapshot_load("<the id>")
f.to_json() == foo.to_json()
Foo.from_json(f.to_json())
```

## TODO

- [x] Use Pyo3 bindings to make this a module available in Python.  
- [ ] Add Makefile  
    - [ ] Add `benchmark` recipe for benchmarking
    - [ ] Add `compare` recipe for comparing benchmarks
- [x] Benchmark  
