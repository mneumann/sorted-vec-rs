use super::{SortedUniqueVec, is_sorted_unique};

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

/// Align the items of two sorted unique vectors.
/// Note: The returned value from `f` must have the same order.
pub fn align<'a, F, T>(a: &'a SortedUniqueVec<T>,
                       b: &'a SortedUniqueVec<T>,
                       f: &mut F)
                       -> SortedUniqueVec<T>
    where F: FnMut(Alignment<'a, T>) -> Option<T>,
          T: Ord + Clone
{
    let mut vec = Vec::with_capacity(a.len() + b.len());

    let mut left_iter = a.as_ref().iter().peekable();
    let mut right_iter = b.as_ref().iter().peekable();
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

    let r = align(&s1,
                  &s2,
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
