use std::clone::Clone;

pub enum IndexType {
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

impl IndexType {
    fn shuffle_order(&self) -> [usize; 3] {
        match self {
            SubjectPredicate => [S, P, O],
            SubjectObject => [S, O, P],
            PredicateSubject => [P, S, O],
            PredicateObject => [P, O, S],
            ObjectSubject => [O, S, P],
            ObjectPredicate => [O, P, S]
        }
    }

    fn shuffle_triple_into<'a, T>(&self, src: &'a[T; 3], dest: &'a mut [&'a T]) {
        let shuffle_order = self.shuffle_order();

        for offset in 0..dest.len() {
            dest[offset] = &src[shuffle_order[offset]];
        }
    }
}