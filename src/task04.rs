use std::cmp::Ordering;

use itertools::Itertools;

use crate::util::CollectIntoVec;

fn either_contains(a: (u64, u64), b: (u64, u64)) -> bool {
    match a.0.cmp(&b.0) {
        Ordering::Equal => true,
        o => o != b.1.cmp(&a.1).reverse(),
    }
}

fn overlap(a: (u64, u64), b: (u64, u64)) -> bool {
    a.0.cmp(&b.0) != a.0.cmp(&b.1) || b.0.cmp(&a.0) != b.0.cmp(&a.1)
}

pub fn task04() {
    let parts = include_str!("../tasks/task04.txt")
        .lines()
        .map(|line| {
            line.split_once(',')
                .map(|(lhs, rhs)| {
                    let m = |s: &str| {
                        s.split_once('-')
                            .map(|(lhs, rhs)| (lhs.parse().unwrap(), rhs.parse().unwrap()))
                            .unwrap()
                    };
                    (m(lhs), m(rhs))
                })
                .map(|(lhs, rhs)| (either_contains(lhs, rhs), overlap(lhs, rhs)))
                .unwrap()
        })
        .fold((0, 0), |(a, b), (c, d)| {
            (a + u64::from(c), b + u64::from(d))
        });

    println!("Part 1: {}", parts.0);
    println!("Part 2: {}", parts.1);
}
