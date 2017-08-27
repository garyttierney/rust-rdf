extern crate rocksdb;
extern crate byteorder;

use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::cmp::Ordering;
use std::marker::PhantomData;
use std::vec::Vec;

use tripledb::IndexKey;

use self::byteorder::ByteOrder;
use self::rocksdb::{DB, Options, ColumnFamily, ColumnFamilyDescriptor, IteratorMode, Direction};

pub trait TableValue: Ord {
    /// Encode this table value to a vector of bytes.
    fn encode(&self) -> Vec<u8>;

    /// Decode an input vector of bytes into an value struct.
    fn decode(data: &[u8]) -> Self;
}

fn cmp_encoded<T: TableValue>(a: &[u8], b: &[u8]) -> Ordering {
    let decoded_a = T::decode(a);
    let decoded_b = T::decode(b);

    return decoded_a.cmp(&decoded_b);
}

/// An iterator over a `Table` that yields `V` values until it reaches the end.
pub struct TableIterator<V: TableValue> {
    _phantom_val: PhantomData<V>
}

pub struct TableDescriptor<K: TableValue, V: TableValue> {
    name: String,
    _phantom_val: PhantomData<V>,
    _phantom_key: PhantomData<K>
}

impl<K: TableValue, V: TableValue> TableDescriptor<K, V> {
    pub fn new<S>(name: S) -> Self where S: Into<String> {
        TableDescriptor {
            name: name.into(),
            _phantom_key: PhantomData,
            _phantom_val: PhantomData
        }
    }

    pub fn to_column_descriptor(&self) -> ColumnFamilyDescriptor {
        let mut options = Options::default();
        options.set_comparator(&self.name[..], cmp_encoded::<K>);

        ColumnFamilyDescriptor::new(&self.name[..], options)
    }

    pub fn open(&self, database: &mut DB) -> Table<K, V> {
        let column_family = database.cf_handle(&self.name[..]).unwrap();

        Table {
            column_family,
            _phantom_val: PhantomData,
            _phantom_key: PhantomData
        }
    }
}

pub struct Table<K: TableValue, V: TableValue> {
    pub column_family: ColumnFamily,
    _phantom_val: PhantomData<V>,
    _phantom_key: PhantomData<K>
}

impl<K: TableValue, V: TableValue> Table<K, V> {}

impl TableValue for u32 {
    fn encode(&self) -> Vec<u8> {
        let mut data: Vec<u8> = vec![0, 0, 0, 0];
        byteorder::LittleEndian::write_u32(&mut data[0..4], *self);

        return data;
    }

    fn decode(data: &[u8]) -> Self {
        return byteorder::LittleEndian::read_u32(data);
    }
}

impl TableValue for String {
    fn encode(&self) -> Vec<u8> {
        return self.as_bytes().to_vec();
    }

    fn decode(data: &[u8]) -> Self {
        return String::from(String::from_utf8_lossy(data));
    }
}
