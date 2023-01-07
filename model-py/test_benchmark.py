from benchmark import rs, py


def test_rs(benchmark):
    benchmark(rs)


def test_py(benchmark):
    benchmark(py)
