pub struct SortedVec<T: Ord + Clone> {
    vec: Vec<T>,
}

fn is_sorted_unique<T: Ord>(slice: &[T]) -> bool {
    if slice.len() < 2 {
        true
    } else {
        slice.windows(2).all(|win| win[0] < win[1])
    }
}

impl<T: Ord + Clone> SortedVec<T> {
    pub fn new() -> Self {
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

    /// Merges `self` and `other` into a new SortedVec.
    pub fn merge(&self, other: &Self) -> Self {
        let mut vec = Vec::with_capacity(self.len() + other.len());

        let mut left_iter = self.vec.iter().peekable();
        let mut right_iter = other.vec.iter().peekable();

        enum Take {
            OneLeft,
            OneRight,
            Both,
            AllLeft,
            AllRight,
        };

        loop {
            let take;

            match (left_iter.peek(), right_iter.peek()) {
                (Some(l), Some(r)) => {
                    if l < r {
                        take = Take::OneLeft;
                    } else if r < l {
                        take = Take::OneRight;
                    } else {
                        take = Take::Both;
                    }
                }
                (Some(_), None) => {
                    take = Take::AllLeft;
                }
                (None, Some(_)) => {
                    take = Take::AllRight;
                }
                (None, None) => {
                    break;
                }
            }
            match take {
                Take::OneLeft => {
                    vec.push((*left_iter.next().unwrap()).clone());
                }
                Take::OneRight => {
                    vec.push((*right_iter.next().unwrap()).clone());
                }
                Take::Both => {
                    // two equal values
                    let left_value = left_iter.next().unwrap();
                    let _right_value = right_iter.next().unwrap();
                    // XXX: decide which one to copy
                    vec.push((*left_value).clone());
                }
                Take::AllLeft => {
                    for item in left_iter {
                        vec.push((*item).clone());
                    }
                    break;
                }
                Take::AllRight => {
                    for item in right_iter {
                        vec.push((*item).clone());
                    }
                    break;
                }

            }
        }

        debug_assert!(is_sorted_unique(&vec));
        SortedVec { vec: vec }
    }
}

impl<T: Ord + Clone> AsRef<[T]> for SortedVec<T> {
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
    assert!(is_sorted_unique(s.as_ref()));
    assert_eq!(&[0, 1, 5, 8], s.as_ref());
    assert_eq!(4, s.len());
}

#[test]
fn test_merge() {
    let mut s1 = SortedVec::new();
    s1.insert(5);
    s1.insert(1);
    s1.insert(8);
    s1.insert(0);
    assert!(is_sorted_unique(s1.as_ref()));

    let mut s2 = SortedVec::new();
    s2.insert(55);
    s2.insert(1);
    s2.insert(5);
    s2.insert(7);
    s2.insert(9);
    assert!(is_sorted_unique(s2.as_ref()));

    let r = s1.merge(&s2);
    assert!(is_sorted_unique(r.as_ref()));
    assert_eq!(&[0, 1, 5, 7, 8, 9, 55], r.as_ref());
}
