#![allow(clippy::cast_possible_truncation)]

use std::collections::HashSet;

use itertools::Itertools;

const OFFSET: u8 = 32;

fn priority(l: u8) -> u8 {
    match l {
        b'a'..=b'z' => l - b'a',
        b'A'..=b'Z' => l - b'A' + 26,
        _ => panic!("Invalid letter"),
    }
}

fn mask(s: &str) -> u128 {
    s.bytes().fold(0, |acc, l| acc | (1 << (l - OFFSET)))
}

pub fn task03() {
    let start = std::time::Instant::now();
    let input = include_str!("../tasks/task03.txt");

    let total_priority = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| {
            let shared_char = mask(a) & mask(b);
            u64::from(priority(shared_char.trailing_zeros() as u8 + OFFSET)) + 1
        })
        .sum::<u64>();

    println!("Part 1: {total_priority}");

    let total_priority = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|c| {
            let shared_char = c.fold(!0, |acc, line| acc & mask(line));
            u64::from(priority(shared_char.trailing_zeros() as u8 + OFFSET)) + 1
        })
        .sum::<u64>();

    println!("Part 2: {total_priority}");
    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}
