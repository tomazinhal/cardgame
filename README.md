## To snapshot a model

I want to create models in rust and write them to Redis.

An Item is a model that contains an ID, name and datetime.
I want to be able to serialize this model to JSON and store it to redis.
I want to get a model from Redis and deserialize it into a model.



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

# Results
After installing `model-py/requirements.txt`, `pytest-benchmark` will be available.

```py
pytest model-py
```

```
-------------------------------------------------------------------------------------------- benchmark: 2 tests -------------------------------------------------------------------------------------------
Name (time in us)            Min                    Max                  Mean                StdDev                Median                 IQR            Outliers         OPS            Rounds  Iterations
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_rs                 311.4550 (1.0)         950.7880 (1.0)        383.1566 (1.0)         74.3232 (1.0)        364.5480 (1.0)       53.5185 (1.0)         49;44  2,609.8992 (1.0)         559           1
test_py               1,291.2380 (4.15)     44,711.5290 (47.03)    2,376.8679 (6.20)     4,374.2392 (58.85)    1,691.2310 (4.64)     318.1212 (5.94)         4;17    420.7217 (0.16)        177           1
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```


## TODO

- [x] Use Pyo3 bindings to make this a module available in Python.  
- [ ] Add Makefile  
- [x] Benchmark  
