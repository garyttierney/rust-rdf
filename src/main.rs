mod tripledb;

use tripledb::StorageEngine;
use tripledb::IndexEntry;

fn main() {
    let mut storage_engine = StorageEngine::open("data/").unwrap();

    let index_entries: Vec<IndexEntry<String>> = vec![
        IndexEntry {
            components: vec![String::from("a"), String::from("b"), String::from("c")],
        },
    ];

    storage_engine.index(index_entries).unwrap();
}
