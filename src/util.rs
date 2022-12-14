use std::{cmp::Ordering, mem::MaybeUninit};

pub trait CollectIntoVec<T>: Iterator<Item = T> {
    fn vec(self) -> Vec<T>;
}

impl<T, I: Iterator<Item = T>> CollectIntoVec<T> for I {
    fn vec(self) -> Vec<T> {
        self.collect()
    }
}

unsafe fn transmute_array<const SIZE: usize, T, U>(arr: [T; SIZE]) -> [U; SIZE] {
    let ptr = std::ptr::addr_of!(arr).cast::<[U; SIZE]>();
    let out = ptr.read();
    std::mem::forget(arr);
    out
}

pub fn borrow_indexes<T>(slice: &mut [T], i1: usize, i2: usize) -> (&mut T, &mut T) {
    if i1 < i2 {
        let split = slice.split_at_mut(i2);
        (&mut split.0[i1], &mut split.1[0])
    } else {
        let split = slice.split_at_mut(i1);
        (&mut split.1[0], &mut split.0[i2])
    }
}

pub trait MinMaxN<T>: Iterator<Item = T>
{
    fn max_n_by(self, n: usize, cmp: impl Fn(&T, &T) -> Ordering) -> Option<Vec<T>>;
    fn max_n_ct_by<const N: usize>(self, cmp: impl Fn(&T, &T) -> Ordering) -> Option<[T; N]>;
    fn max_n(self, n: usize) -> Option<Vec<T>> where T: Ord ;
    fn min_n(self, n: usize) -> Option<Vec<T>> where T: Ord;
    fn max_n_ct<const N: usize>(self) -> Option<[T; N]> where T: Ord;
    fn min_n_ct<const N: usize>(self) -> Option<[T; N]> where T: Ord;
}

impl<T, I: Iterator<Item = T>> MinMaxN<T> for I
{
    fn max_n_by(mut self, n: usize, cmp: impl Fn(&T, &T) -> Ordering) -> Option<Vec<T>> {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(self.next()?);
        }
        v.sort_unstable_by(|a, b| cmp(b, a));
        for x in self {
            if let Some(last) = v.last_mut() {
                if cmp(&x, last) == Ordering::Greater {
                    *last = x;
                    v.sort_unstable_by(|a, b| cmp(b, a));
                }
            }
        }
        Some(v)
    }

    fn max_n(self, n: usize) -> Option<Vec<T>> where T: Ord {
        self.max_n_by(n, Ord::cmp)
    }

    fn min_n(self, n: usize) -> Option<Vec<T>> where T: Ord {
        self.max_n_by(n, |a, b| b.cmp(a))
    }

    fn max_n_ct_by<const N: usize>(mut self, cmp: impl Fn(&T, &T) -> Ordering) -> Option<[T; N]> {
        // we would love to use `let mut v = array::try_from_fn(|_| self.next())?;`, but that's unstable.

        // Create an uninitialized array of `MaybeUninit`. The `assume_init` is
        // safe because the type we are claiming to have initialized here is a
        // bunch of `MaybeUninit`s, which do not require initialization.
        let mut v = unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() };
        for loc in v.iter_mut() {
            loc.write(self.next()?);
        }
        // if we have made it here, then every element in v is initialized
        // so we can now convert it to a slice of initialized elements
        let mut v = unsafe { transmute_array::<N, MaybeUninit<T>, T>(v) };
        v.sort_unstable_by(|a, b| cmp(b, a));
        for x in self {
            if let Some(last) = v.last_mut() {
                if cmp(&x, last) == Ordering::Greater {
                    *last = x;
                    v.sort_unstable_by(|a, b| cmp(b, a));
                }
            }
        }
        Some(v)
    }

    fn max_n_ct<const N: usize>(self) -> Option<[T; N]> where T: Ord {
        self.max_n_ct_by::<N>(Ord::cmp)
    }

    fn min_n_ct<const N: usize>(self) -> Option<[T; N]> where T: Ord {
        self.max_n_ct_by::<N>(|a, b| b.cmp(a))
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_max_n() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(v.iter().max_n(3), Some(vec![&10, &9, &8]));
        assert_eq!(v.iter().max_n(0), Some(vec![]));
        assert_eq!(v.iter().max_n(11), None);
    }

    #[test]
    fn test_min_n() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(v.iter().min_n(3), Some(vec![&1, &2, &3]));
        assert_eq!(v.iter().min_n(0), Some(vec![]));
        assert_eq!(v.iter().min_n(11), None);
    }

    #[test]
    fn test_max_n_ct() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(v.iter().max_n_ct::<3>(), Some([&10, &9, &8]));
        assert_eq!(v.iter().max_n_ct::<0>(), Some([]));
        assert_eq!(v.iter().max_n_ct::<11>(), None);
    }

    #[test]
    fn test_min_n_ct() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(v.iter().min_n_ct::<3>(), Some([&1, &2, &3]));
        assert_eq!(v.iter().min_n_ct::<0>(), Some([]));
        assert_eq!(v.iter().min_n_ct::<11>(), None);
    }

    #[test]
    fn test_max_n_ct_by() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(
            v.iter().max_n_ct_by::<3>(|a, b| b.cmp(a)),
            Some([&1, &2, &3])
        );
        assert_eq!(v.iter().max_n_ct_by::<0>(|a, b| b.cmp(a)), Some([]));
        assert_eq!(v.iter().max_n_ct_by::<11>(|a, b| b.cmp(a)), None);
    }
}
