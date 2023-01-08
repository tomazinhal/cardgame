import pytest


def pytest_addoption(parser):
    parser.addoption("--items", action="store", default=1)


@pytest.fixture(scope="session")
def num_items(pytestconfig):
    return int(pytestconfig.getoption("items"))
