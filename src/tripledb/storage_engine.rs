extern crate rocksdb;

use self::rocksdb::DB;
use self::rocksdb::Error;
use self::rocksdb::Options;
use self::rocksdb::WriteBatch;
use self::rocksdb::WriteOptions;

use std::path::Path;
use std::borrow::BorrowMut;

use tripledb::IndexKey;
use tripledb::IndexType;
use tripledb::Table;
use tripledb::TableDescriptor;
use tripledb::TableValue;

pub struct StorageEngine {
    database: DB,
    id_table: Table<String, u32>,
    uri_table: Table<u32, String>,
    counter: u32
}

const ID_TABLE_NAME: &str = "id_table";
const URI_TABLE_NAME: &str = "uri_table";

impl StorageEngine {
    pub fn open(path: &Path) -> Result<StorageEngine, String> {
        let mut options = rocksdb::Options::default();
        options.create_if_missing(true);

        let id_table_descriptor: TableDescriptor<String, u32> = TableDescriptor::new(ID_TABLE_NAME);
        let uri_table_descriptor: TableDescriptor<u32, String> = TableDescriptor::new(URI_TABLE_NAME);

        let mut column_families = vec![];
        column_families.push(id_table_descriptor.to_column_descriptor());
        column_families.push(uri_table_descriptor.to_column_descriptor());

        let mut database : DB = DB::open_cf_descriptors(&options, path, column_families)?;

        let id_table: Table<String, u32> = id_table_descriptor.open(&mut database);
        let uri_table: Table<u32, String> = uri_table_descriptor.open(&mut database);

        return Ok(StorageEngine {
            database,
            id_table,
            uri_table,
            counter: 0
        });
    }

    pub fn store(&self, value: String) -> Result<(), Error> {
        let mut batch = WriteBatch::default();
        let encoded_counter = self.counter.encode();
        let encoded_value = value.encode();

        try!(batch.put_cf(self.id_table.column_family, &encoded_value, &encoded_counter));
        try!(batch.put_cf(self.uri_table.column_family, &encoded_counter, &encoded_value));

        return self.database.write(batch);
    }
}
