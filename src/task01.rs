use crate::util::get_task;

pub fn task01() {
    let data = get_task(1);

    let top3 = data
        .split("\r\n\r\n")
        .map(|run| {
            run.trim()
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum()
        })
        .fold([0, 0, 0], |mut top_3, num| {
            if num > top_3[2] {
                top_3[2] = num;
                top_3.sort_unstable_by(|a, b| b.cmp(a));
            }
            top_3
        });

    println!("Part 1: {}", top3[0]);
    println!("Part 2: {}", top3.iter().sum::<u64>());
}
