pub struct SortedVec<T: Ord> {
    vec: Vec<T>,
}

impl<T: Ord> SortedVec<T> {
    pub fn new() -> SortedVec<T> {
        SortedVec { vec: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn contains(&self, element: &T) -> bool {
        self.vec.binary_search(element).is_ok()
    }

    /// Insert `element` into the sorted list.
    /// 
    /// Panics if an element with the same key (according to the Eq trait) already exists.
    pub fn insert(&mut self, element: T) {
        match self.vec.binary_search(&element) {
            Ok(_idx) => {
                panic!("Element already exists");
            }
            Err(idx) => {
                self.vec.insert(idx, element);
            }
        }
    }
}

impl<T: Ord> AsRef<[T]> for SortedVec<T> {
    fn as_ref(&self) -> &[T] {
        &self.vec
    }
}

#[test]
fn test_insert() {
    let mut s = SortedVec::new();
    s.insert(5);
    s.insert(1);
    s.insert(8);
    s.insert(0);
    assert_eq!(&[0, 1, 5, 8], s.as_ref());
    assert_eq!(4, s.len());
}
