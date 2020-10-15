use pyo3::ffi::PyVarObject;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyString};
use sled::{Config, Db, Error, IVec, Result, Tree};
use std::str;

#[pyclass]
struct Cache {
    db: Db,
}

#[pymethods]
impl Cache {
    #[new]
    pub fn __new__() -> Self {
        let config = Config::new().temporary(true);
        let db = config.open().unwrap();
        Cache {
            db
        }
    }

    pub fn set(&mut self, key: &PyString, value: &PyString) {
        let temp_key = key.to_str();
        let temp_val = value.to_str();
        let v = temp_val.unwrap().as_bytes().to_vec();
        let k = temp_key.unwrap().as_bytes().to_vec();
        self.db.insert(k, v).unwrap();
    }
    pub fn get(&mut self, key:&PyString) -> String {
        let temp_key = key.to_str().unwrap().as_bytes().to_vec();
        let buf = match Tree::get(&self.db, temp_key) {
            Ok(T) => T,
            Err(_) => None
        };
        if buf != None {
            let z = buf.unwrap().to_vec();
            let string_result = match str::from_utf8(&z) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            string_result.to_string()
        } else {
            "".to_string()
        }

    }
}

#[pymodule]
pub fn utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Cache>();
    Ok(())
}



