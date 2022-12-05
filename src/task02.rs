#![allow(clippy::cast_possible_wrap)]

use crate::util::get_task;

const fn choose_move(them: u8, outcome: u8) -> u8 { ((them as i8 + (outcome as i8 - 1)).rem_euclid(3)) as u8 }
const fn outcome_score(them: u8, us: u8) -> u8 { [3, 0, 6][(them as i8 - us as i8).rem_euclid(3) as usize] }

pub fn task02() {
    let start = std::time::Instant::now();
    let scores = include_str!("../tasks/task02.txt")
        .lines()
        .map(|line| {
            let b = line.as_bytes();
            let them = b[0] - b'A';
            let us_or_outcome = b[2] - b'X';
            (
                u64::from(1 + us_or_outcome + outcome_score(them, us_or_outcome)), // part 1
                u64::from(1 + choose_move(them, us_or_outcome) + us_or_outcome * 3), // part 2
            )
        })
        .reduce(|(a, b), (c, d)| (a + c, b + d))
        .unwrap();

    println!("Part 1: {}", scores.0);
    println!("Part 2: {}", scores.1);
    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}
