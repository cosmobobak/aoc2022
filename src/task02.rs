use crate::util::get_task;

fn outcome_score(them: u8, us: u8) -> u8 {
    // 0 is rock, 1 is paper, 2 is scissors
    match (them, us) {
        (0, 0) | (1, 1) | (2, 2) => 3,
        (0, 1) | (1, 2) | (2, 0) => 6,
        (0, 2) | (1, 0) | (2, 1) => 0,
        _ => panic!("Invalid input"),
    }
}

fn choose_move(them: u8, outcome: u8) -> u8 {
    // 0 is rock, 1 is paper, 2 is scissors
    match (them, outcome) {
        (x, 1) => x,
        (0, 2) | (2, 0) => 1,
        (1, 2) | (0, 0) => 2,
        (2, 2) | (1, 0) => 0,
        _ => panic!("Invalid input"),
    }
}

pub fn task02() {
    let data = get_task(2);

    let scores = data
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
}
