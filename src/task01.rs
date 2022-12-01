use crate::util::get_task;

pub fn task01() {
    let data = get_task(1);

    let runs = data.split("\r\n\r\n").collect::<Vec<&str>>();
    let nums = runs.iter()
        .map(|run| run.trim().lines().map(|line| {
            line.parse::<u64>().unwrap()
        }).sum::<u64>()).collect::<Vec<u64>>();

    println!("Task 1: {}", nums.iter().max().unwrap());

    let top_3 = nums.into_iter().fold([0, 0, 0], |mut top_3, num| {
        if num > top_3[2] {
            top_3[2] = num;
            top_3.sort_unstable_by(|a, b| b.cmp(a));
        }
        top_3
    });

    println!("Task 2: {}", top_3.iter().sum::<u64>());
}