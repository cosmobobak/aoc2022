use std::cmp::Ordering;

use itertools::Itertools;

use crate::util::CollectIntoVec;

const fn either_contains(a: (i32, i32), b: (i32, i32)) -> bool {
    let left_vector = b.0 - a.0;
    let right_vector = b.1 - a.1;
    left_vector * right_vector <= 0
}

const fn overlap(a: (i32, i32), b: (i32, i32)) -> bool {
    let left_vector = b.0 - a.1;
    let right_vector = b.1 - a.0;
    left_vector * right_vector <= 0
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
