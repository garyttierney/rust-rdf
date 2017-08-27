use std::cmp::Ordering;
use std::fmt::{self, Display};

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub enum IndexKeyType {
    SubjectPredicate,
    SubjectObject,
    PredicateSubject,
    PredicateObject,
    ObjectSubject,
    ObjectPredicate,
}

const S: usize = 0;
const P: usize = 1;
const O: usize = 2;

impl Display for IndexKeyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

const INDEX_TYPES: [IndexKeyType; 6] = [
    IndexKeyType::SubjectPredicate,
    IndexKeyType::SubjectObject,
    IndexKeyType::PredicateSubject,
    IndexKeyType::PredicateObject,
    IndexKeyType::ObjectSubject,
    IndexKeyType::ObjectPredicate,
];

impl IndexKeyType {
    fn shuffle_order(&self) -> [usize; 3] {
        match self {
            SubjectPredicate => [S, P, O],
            SubjectObject => [S, O, P],
            PredicateSubject => [P, S, O],
            PredicateObject => [P, O, S],
            ObjectSubject => [O, S, P],
            ObjectPredicate => [O, P, S],
        }
    }

    pub fn id(&self) -> u8 {
        match self {
            SubjectPredicate => 0,
            SubjectObject => 1,
            PredicateSubject => 2,
            PredicateObject => 3,
            ObjectSubject => 4,
            ObjectPredicate => 5,
        }
    }

    pub fn values() -> [IndexKeyType; 6] {
        INDEX_TYPES
    }

    pub fn shuffle_triple_into<'a, T: Copy>(&self, src: &'a [T], dest: &'a mut [T]) {
        let shuffle_order = self.shuffle_order();

        for offset in 0..dest.len() {
            dest[offset] = src[shuffle_order[offset]];
        }
    }
}

impl Ord for IndexKeyType {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.id().cmp(&other.id());
    }
}

pub struct IndexEntry<T: Sized> {
    pub components: Vec<T>,
}

impl<T: Sized> IndexEntry<T> {
    pub fn components(&self) -> &[T] {
        return &self.components;
    }

    pub fn map<O: Sized, F>(&self, mut mapper: F) -> IndexEntry<O>
    where
        F: FnMut(&T) -> O,
    {
        let mut components = Vec::with_capacity(3);

        for offset in 0..3 {
            let output = mapper(&self.components[offset as usize]);
            components.insert(offset as usize, output);
        }

        IndexEntry { components }
    }
}
