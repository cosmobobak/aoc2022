use crate::util::MinMaxN;

pub fn task01() {
    let start = std::time::Instant::now();
    let top3 = include_str!("../tasks/task01.txt")
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
    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}
