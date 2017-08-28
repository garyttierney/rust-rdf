mod tripledb;

use tripledb::IndexEntry;
use tripledb::StorageEngine;

fn main() {
    let mut storage_engine = StorageEngine::open("data/").unwrap();

    let index_entries: Vec<IndexEntry<String>> = vec![IndexEntry::from(["a", "b", "c"])];

    if storage_engine.index(index_entries).is_err() {
        panic!("Failed to add entries to index");
    }
}
