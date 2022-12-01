// use adqselect::nth_element;

use std::{cmp::Ordering, mem::MaybeUninit};

pub fn get_task(task_id: usize) -> String {
    std::fs::read_to_string(format!("tasks/task{:02}.txt", task_id)).expect("Error in file fetch.")
}

// pub fn median_element<T: Ord>(s: &mut [T]) -> &mut T {
//     // mutates the input slice in order to find
//     // the median element in O(n) time
//     let idx = s.len() / 2;
//     nth_element(s, idx, &mut Ord::cmp);
//     &mut s[idx]
// }

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

pub trait MinMaxN<T>: Iterator<Item = T>
where
    T: Ord,
{
    fn max_n_by(self, n: usize, cmp: impl Fn(&T, &T) -> Ordering) -> Option<Vec<T>>;
    fn max_n(self, n: usize) -> Option<Vec<T>>;
    fn min_n(self, n: usize) -> Option<Vec<T>>;
    fn max_n_ct_by<const N: usize>(self, cmp: impl Fn(&T, &T) -> Ordering) -> Option<[T; N]>;
    fn max_n_ct<const N: usize>(self) -> Option<[T; N]>;
    fn min_n_ct<const N: usize>(self) -> Option<[T; N]>;
}

impl<T, I: Iterator<Item = T>> MinMaxN<T> for I
where
    T: Ord,
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

    fn max_n(self, n: usize) -> Option<Vec<T>> {
        self.max_n_by(n, Ord::cmp)
    }

    fn min_n(self, n: usize) -> Option<Vec<T>> {
        self.max_n_by(n, |a, b| b.cmp(a))
    }

    fn max_n_ct_by<const N: usize>(
        mut self,
        cmp: impl Fn(&T, &T) -> Ordering,
    ) -> Option<[T; N]> {
        // Create an uninitialized array of `MaybeUninit`. The `assume_init` is
        // safe because the type we are claiming to have initialized here is a
        // bunch of `MaybeUninit`s, which do not require initialization.
        let mut v = unsafe {
            MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init()
        };
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

    fn max_n_ct<const N: usize>(self) -> Option<[T; N]> {
        self.max_n_ct_by::<N>(Ord::cmp)
    }

    fn min_n_ct<const N: usize>(self) -> Option<[T; N]> {
        self.max_n_ct_by::<N>(|a, b| b.cmp(a))
    }
}
