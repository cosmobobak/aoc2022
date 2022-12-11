#![allow(clippy::cast_sign_loss)]

pub fn task10() {
    let start = std::time::Instant::now();
    let text = include_str!("../tasks/task10.txt");

    let mut x = 1;
    let mut clocks = 1;
    let mut cycle_sum = 0;
    let mut display = [0; 6 * 40];
    let is_drawn = |register: i32, clock: i32| ((clock - 1) % 40).abs_diff(register) <= 1;
    let mut cycle = |r| {
        if clocks - 20 >= 0 && (clocks - 20) % 40 == 0 {
            cycle_sum += r * clocks;
        }
        display[clocks as usize - 1] = [b' ', b'@'][usize::from(is_drawn(r, clocks))];
        clocks += 1;
    };

    for instruction in text.lines() {
        match instruction {
            "noop" => cycle(x),
            addx => {
                cycle(x);
                cycle(x);
                let value = addx[5..].parse::<i32>().unwrap();
                x += value;
            }
        }
    }

    println!("Part 1: {cycle_sum}");
    println!("Part 2:");
    for row in display.chunks(40) {
        println!("{}", std::str::from_utf8(row).unwrap());
    }

    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}
