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
        match *self {
            IndexKeyType::SubjectPredicate => [S, P, O],
            IndexKeyType::SubjectObject => [S, O, P],
            IndexKeyType::PredicateSubject => [P, S, O],
            IndexKeyType::PredicateObject => [P, O, S],
            IndexKeyType::ObjectSubject => [O, S, P],
            IndexKeyType::ObjectPredicate => [O, P, S],
        }
    }

    pub fn id(&self) -> u8 {
        match *self {
            IndexKeyType::SubjectPredicate => 0,
            IndexKeyType::SubjectObject => 1,
            IndexKeyType::PredicateSubject => 2,
            IndexKeyType::PredicateObject => 3,
            IndexKeyType::ObjectSubject => 4,
            IndexKeyType::ObjectPredicate => 5,
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
        self.id().cmp(&other.id())
    }
}

pub struct IndexEntry<T: Sized> {
    pub components: Vec<T>,
}

impl<T: Sized> IndexEntry<T> {
    /// Create an `IndexEntry` from an array of components.
    ///
    /// # Arguments
    ///
    /// * `src_components` - An array of components that can be converted `Into` `T`.
    pub fn from<S>(src_components: [S; 3]) -> IndexEntry<T>
    where
        S: Into<T> + Copy,
    {
        let mut components = Vec::with_capacity(3);
        for &component in &src_components {
            components.push(component.into());
        }

        IndexEntry { components }
    }

    /// Get the components of this `IndexEntry`.
    pub fn components(&self) -> &[T] {
        &self.components
    }

    /// Use a mapping function to convert this entries components into a new `IndexEntry`.
    ///
    /// # Arguments
    ///
    /// * `mapper` - A closure that returns a value `O` for a value `T`.
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
