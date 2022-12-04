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
    let (mut part1, mut part2): (u64, u64) = (0, 0);

    for line in include_str!("../tasks/task04.txt").lines() {
        let (lhs, rhs) = line
            .split_once(',')
            .map(|(lhs, rhs)| {
                let m = |s: &str| {
                    s.split_once('-')
                        .map(|(lhs, rhs)| (lhs.parse().unwrap(), rhs.parse().unwrap()))
                        .unwrap()
                };
                (m(lhs), m(rhs))
            })
            .unwrap();
        part1 += u64::from(either_contains(lhs, rhs));
        part2 += u64::from(overlap(lhs, rhs));
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
