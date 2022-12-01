use crate::util::{get_task, MinMaxN};

pub fn task01() {
    let data = get_task(1); // the raw text of the input file

    let top3 = data
        .split("\r\n\r\n")
        .map(|run| {
            run.trim()
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum()
        })
        .max_n_ct::<3>()
        .unwrap();

    println!("Part 1: {}", top3[0]);
    println!("Part 2: {}", top3.iter().sum::<u64>());
}
