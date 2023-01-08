use pyo3::prelude::*;
mod common;
use common::models::{Item, ItemList};
use common::snapshot::{Cache, Snapshot};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
impl Snapshot for ItemList {
    fn get_key(&self) -> String {
        self.id.to_string()
    }
}
impl Cache for ItemList {}

#[pyclass]
struct Foo {
    item_list: ItemList,
}

#[pymethods]
impl Foo {
    #[new]
    fn new(new_name: &str) -> Foo {
        Foo {
            item_list: ItemList::new(new_name),
        }
    }

    fn get_id(&self) -> PyResult<String> {
        Ok(self.item_list.get_key())
    }

    fn add_item(&mut self, item_name: &str) {
        self.item_list.add_item(Item::new(item_name))
    }

    fn snapshot_store(&self) -> PyResult<bool> {
        let _ = self.item_list.store();
        Ok(true)
    }

    #[staticmethod]
    fn snapshot_load(id: &str) -> Foo {
        Foo {
            item_list: ItemList::load(String::from(id)),
        }
    }

    fn to_json(&self) -> PyResult<String> {
        Ok(self.item_list.to_str())
    }

    #[staticmethod]
    fn from_json(json: &str) -> Foo {
        Foo {
            item_list: ItemList::from_str(json),
        }
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn moders(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<Foo>()?;
    Ok(())
}
