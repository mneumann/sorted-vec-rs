use std::ops::Deref;

/// A `Vec` in sorted order without duplicates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SortedUniqueVec<T: Ord + Clone> {
    vec: Vec<T>,
}

fn is_sorted_unique<T: Ord>(slice: &[T]) -> bool {
    if slice.len() < 2 {
        true
    } else {
        slice.windows(2).all(|win| win[0] < win[1])
    }
}

pub enum LeftOrRight {
    Left,
    Right,
}

#[derive(Debug)]
pub enum Alignment<'a, T: 'a> {
    Match(&'a T, &'a T),
    ExcessLeftHead(&'a T),
    ExcessRightHead(&'a T),
    ExcessLeftTail(&'a T),
    ExcessRightTail(&'a T),
    DisjointLeft(&'a T),
    DisjointRight(&'a T),
}

impl<T: Ord + Clone> SortedUniqueVec<T> {
    pub fn new() -> Self {
        SortedUniqueVec { vec: Vec::new() }
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

    /// Align the items of two sorted unique vectors.
    /// Note: The returned value from `f` must have the same order.
    pub fn align<'a, F>(&'a self, other: &'a Self, f: &mut F) -> Self
        where F: FnMut(Alignment<'a, T>) -> Option<T>
    {
        let mut vec = Vec::with_capacity(self.len() + other.len());

        let mut left_iter = self.vec.iter().peekable();
        let mut right_iter = other.vec.iter().peekable();
        let mut left_count = 0;
        let mut right_count = 0;

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
                    let value = left_iter.next().unwrap();

                    let alignment = if right_count == 0 {
                        Alignment::ExcessLeftHead(value)
                    } else {
                        Alignment::DisjointLeft(value)
                    };

                    if let Some(new) = f(alignment) {
                        assert!(value.eq(&new));
                        vec.push(new);
                    }

                    left_count += 1;
                }
                Take::OneRight => {
                    let value = right_iter.next().unwrap();

                    let alignment = if left_count == 0 {
                        Alignment::ExcessRightHead(value)
                    } else {
                        Alignment::DisjointRight(value)
                    };

                    if let Some(new) = f(alignment) {
                        assert!(value.eq(&new));
                        vec.push(new);
                    }

                    right_count += 1;
                }
                Take::Both => {
                    // two equal values
                    let left_value = left_iter.next().unwrap();
                    let right_value = right_iter.next().unwrap();
                    debug_assert!(left_value.eq(right_value));

                    let alignment = Alignment::Match(left_value, right_value);

                    if let Some(new) = f(alignment) {
                        assert!(left_value.eq(&new));
                        vec.push(new);
                    }

                    left_count += 1;
                    right_count += 1;
                }
                Take::AllLeft => {
                    // There are no items left on the right side, so all items are ExcessLeftTail.
                    for item in left_iter {
                        let alignment = Alignment::ExcessLeftTail(item);
                        if let Some(new) = f(alignment) {
                            assert!(item.eq(&new));
                            vec.push(new);
                        }
                    }
                    break;
                }
                Take::AllRight => {
                    // There are no items left on the right side, so all items are ExcessRightTail.
                    for item in right_iter {
                        let alignment = Alignment::ExcessRightTail(item);
                        if let Some(new) = f(alignment) {
                            assert!(item.eq(&new));
                            vec.push(new);
                        }
                    }
                    break;
                }
            }
        }

        debug_assert!(is_sorted_unique(&vec));
        SortedUniqueVec { vec: vec }
    }

    /// Merges `self` and `other` into a new SortedVec.
    /// `choose_equal` decides which one of two equal values to take.
    pub fn merge<F>(&self, other: &Self, choose_equal: &F) -> Self
        where F: Fn(&T, &T) -> LeftOrRight
    {
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
                    let right_value = right_iter.next().unwrap();
                    match choose_equal(left_value, right_value) {
                        LeftOrRight::Left => {
                            vec.push((*left_value).clone());
                        }
                        LeftOrRight::Right => {
                            vec.push((*right_value).clone());
                        }
                    }
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
        SortedUniqueVec { vec: vec }
    }
}

impl<T: Ord + Clone> AsRef<[T]> for SortedUniqueVec<T> {
    fn as_ref(&self) -> &[T] {
        &self.vec
    }
}

impl<T: Ord + Clone> Deref for SortedUniqueVec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.vec
    }
}

#[test]
fn test_insert() {
    let mut s = SortedUniqueVec::new();
    s.insert(5);
    s.insert(1);
    s.insert(8);
    s.insert(0);
    assert!(is_sorted_unique(&s));
    assert_eq!(&[0, 1, 5, 8][..], s.as_ref());
    assert_eq!(4, s.len());
}

#[test]
fn test_merge() {
    let mut s1 = SortedUniqueVec::new();
    s1.insert(5);
    s1.insert(1);
    s1.insert(8);
    s1.insert(0);
    assert!(is_sorted_unique(&s1));

    let mut s2 = SortedUniqueVec::new();
    s2.insert(55);
    s2.insert(1);
    s2.insert(5);
    s2.insert(7);
    s2.insert(9);
    assert!(is_sorted_unique(&s2));

    let r = s1.merge(&s2, &|_, _| LeftOrRight::Left);
    assert!(is_sorted_unique(&r));
    assert_eq!(&[0, 1, 5, 7, 8, 9, 55][..], r.as_ref());
}

#[test]
fn test_align() {
    let mut s1 = SortedUniqueVec::new();
    s1.insert(0);
    s1.insert(1);
    s1.insert(5);
    s1.insert(8);
    assert!(is_sorted_unique(&s1));

    let mut s2 = SortedUniqueVec::new();
    s2.insert(1);
    s2.insert(5);
    s2.insert(7);
    s2.insert(9);
    s2.insert(55);
    assert!(is_sorted_unique(&s2));

    let r = s1.align(&s2,
                     &mut |alignment| {
                         match alignment {
                             Alignment::Match(a, _b) => Some((*a).clone()),
                             Alignment::ExcessLeftHead(a) |
                             Alignment::ExcessRightHead(a) |
                             Alignment::ExcessLeftTail(a) |
                             Alignment::ExcessRightTail(a) |
                             Alignment::DisjointLeft(a) |
                             Alignment::DisjointRight(a) => Some((*a).clone()),
                         }
                     });

    assert!(is_sorted_unique(&r));
    assert_eq!(&[0, 1, 5, 7, 8, 9, 55][..], r.as_ref());
}
