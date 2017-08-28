extern crate byteorder;

use self::byteorder::ByteOrder;
use std::cmp::Ordering;
use tripledb::TableValue;

const INDEX_KEY_LENGTH: usize = 3;
const INDEX_KEY_BYTES: usize = 12;

#[derive(Eq, PartialOrd, PartialEq)]
pub struct IndexKey {
    components: [u32; INDEX_KEY_LENGTH],
}

impl Ord for IndexKey {
    fn cmp(&self, other: &Self) -> Ordering {
        for offset in 0..INDEX_KEY_LENGTH {
            let a = self.components[offset];
            let b = other.components[offset];
            let comparison = a.cmp(&b);

            if comparison != Ordering::Equal {
                return comparison;
            }
        }

        Ordering::Equal
    }
}

impl TableValue for IndexKey {
    fn decode(key_data: &[u8]) -> IndexKey {
        let mut components = [0; INDEX_KEY_LENGTH];
        byteorder::LittleEndian::read_u32_into(key_data, &mut components);

        IndexKey { components }
    }

    fn encode(&self) -> Vec<u8> {
        let mut data = vec![0; INDEX_KEY_BYTES];
        byteorder::LittleEndian::write_u32_into(&self.components, &mut data[0..INDEX_KEY_BYTES]);

        data
    }
}

impl IndexKey {
    pub fn from(components: [u32; INDEX_KEY_LENGTH]) -> Self {
        IndexKey { components }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_correctly() {
        let a_components = vec![[0, 0, 0], [0, 1, 0], [0, 1, 1]];
        let b_components = vec![[1, 0, 0], [0, 1, 0], [0, 1, 0]];
        let expectations = [Ordering::Less, Ordering::Equal, Ordering::Greater];

        for offset in 0..3 {
            let a = IndexKey::from(a_components[offset]);
            let b = IndexKey::from(b_components[offset]);
            assert_eq!(expectations[offset], a.cmp(&b));
        }
    }
}
