mod tripledb;

use std::path::Path;
use tripledb::StorageEngine;
use tripledb::Table;

fn main() {
    let path = Path::new("data/");
    let storage_engine = StorageEngine::open(&path).unwrap();
    let value = String::from("test");


    storage_engine.store(value);
}
