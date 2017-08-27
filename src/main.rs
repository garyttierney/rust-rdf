mod tripledb;

use std::path::Path;
use tripledb::StorageEngine;
use tripledb::Table;
use tripledb::IndexEntry;

fn main() {
    let path = Path::new("data/");
    let mut storage_engine = StorageEngine::open(&path).unwrap();

    let index_entries: Vec<IndexEntry<String>> = vec![
        IndexEntry {
            components: vec![String::from("a"), String::from("b"), String::from("c")],
        },
    ];

    storage_engine.index(index_entries);
}
