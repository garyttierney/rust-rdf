extern crate rocksdb;

use self::rocksdb::DB;
use self::rocksdb::Error;
use self::rocksdb::WriteBatch;

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::path::Path;

use tripledb::IndexEntry;
use tripledb::IndexKey;
use tripledb::IndexKeyType;
use tripledb::Table;
use tripledb::TableDescriptor;
use tripledb::TableValue;

type IndexTable = Table<IndexKey, u32>;

pub struct StorageEngine {
    database: Arc<RwLock<DB>>,
    id_table: Table<String, u32>,
    uri_table: Table<u32, String>,
    index_tables: BTreeMap<u8, IndexTable>,
    counter: u32,
}

const ID_TABLE_NAME: &str = "id_table";
const URI_TABLE_NAME: &str = "uri_table";

impl StorageEngine {
    /// Open a new `StorageEngine` from the given path,
    /// creating the database if it doesn't already exist.
    pub fn open(path: &Path) -> Result<StorageEngine, String> {
        let mut options = rocksdb::Options::default();
        options.create_if_missing(true);
        options.create_missing_column_families(true);

        let id_table_descriptor = TableDescriptor::new(ID_TABLE_NAME);
        let uri_table_descriptor = TableDescriptor::new(URI_TABLE_NAME);

        let index_key_types = IndexKeyType::values();
        let index_table_descriptors: Vec<(u8, TableDescriptor<IndexKey, u32>)> = index_key_types
            .iter()
            .map(|key_type| {
                (key_type.id(), TableDescriptor::new(key_type.to_string()))
            })
            .collect();

        let mut column_families = vec![];
        column_families.push(id_table_descriptor.to_column_descriptor());
        column_families.push(uri_table_descriptor.to_column_descriptor());

        for &(_, ref descriptor) in &index_table_descriptors {
            column_families.push(descriptor.to_column_descriptor());
        }

        let db = DB::open_cf_descriptors(&options, path, column_families)?;
        let db_lock = Arc::new(RwLock::new(db));

        let id_table = id_table_descriptor.open(&db_lock);
        let uri_table = uri_table_descriptor.open(&db_lock);
        let index_tables: BTreeMap<_, _> = index_table_descriptors
            .into_iter()
            .map(|(id, ref desc)| (id, desc.open(&db_lock)))
            .collect();

        return Ok(StorageEngine {
            database: db_lock,
            id_table,
            uri_table,
            index_tables,
            counter: 0,
        });
    }

    /// Consume a collection of triples into the storage engines indexes.
    pub fn index(&mut self, entries: Vec<IndexEntry<String>>) -> Result<(), String> {
        let mut batch = WriteBatch::default();

        let encoded_entries: Vec<IndexEntry<u32>> = entries
            .into_iter()
            .map(|entry| {
                entry.map(|val| self.store(val.into(), &mut batch).unwrap())
            })
            .collect();

        let index_value = 0;
        let index_key_types = IndexKeyType::values();
        let mut index_key_components: [u32; 3] = [0; 3];

        for (type_id, table) in self.index_tables.iter() {
            let index_key_type = &index_key_types[*type_id as usize];

            for entry in encoded_entries.iter() {
                index_key_type.shuffle_triple_into(
                    entry.components(),
                    &mut index_key_components[..],
                );

                table.put(
                    &mut batch,
                    &IndexKey::from(index_key_components),
                    &index_value,
                );
            }
        }

        let database_writer = self.database.write().unwrap();
        match database_writer.write(batch) {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("eek"))
        }
    }

    /// Store a single `String` value in the underlying storage engine
    /// and allocate a unique `u32` value for it.
    fn store(&mut self, value: &String, batch: &mut WriteBatch) -> Result<u32, &str> {
        if let Ok(Some(val)) = self.id_table.get(value) {
            return Ok(val);
        }

        let counter = self.counter;
        self.id_table.put(batch, &value, &counter);
        self.uri_table.put(batch, &counter, &value);

        self.counter += 1;
        return Ok(counter);
    }
}
