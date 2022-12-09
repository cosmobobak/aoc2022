use std::collections::HashSet;

const fn movement(
    head @ (head_x, head_y): (i32, i32),
    tail @ (tail_x, tail_y): (i32, i32),
) -> (i32, i32) {
    if touching(head, tail) {
        return tail;
    }
    let dx = (head_x - tail_x).signum();
    let dy = (head_y - tail_y).signum();
    (tail_x + dx, tail_y + dy)
}

const fn touching((head_x, head_y): (i32, i32), (tail_x, tail_y): (i32, i32)) -> bool {
    (head_x - tail_x).abs() <= 1 && (head_y - tail_y).abs() <= 1
}

fn compute<const KNOTS: usize>(text: &str) -> usize {
    let mut rope = [(0, 0); KNOTS];
    let mut tail_positions_seen = std::collections::HashSet::new();
    tail_positions_seen.insert(*rope.last().unwrap());
    for instruction in text.lines() {
        let direction = instruction.as_bytes()[0];
        let times = instruction[2..].parse::<i32>().unwrap();
        for _ in 0..times {
            let head = &mut rope[0];
            match direction {
                b'U' => head.1 -= 1,
                b'D' => head.1 += 1,
                b'L' => head.0 -= 1,
                b'R' => head.0 += 1,
                _ => panic!("Unknown direction: {direction}"),
            }
            for i in 0..KNOTS - 1 {
                rope[i + 1] = movement(rope[i], rope[i + 1]);
            }
            tail_positions_seen.insert(*rope.last().unwrap());
        }
    }
    tail_positions_seen.len()
}

pub fn task09() {
    let start = std::time::Instant::now();
    let text = include_str!("../tasks/task09.txt");

    let p1 = compute::<2>(text);

    println!("Part 1: {p1}");

    let p2 = compute::<10>(text);

    println!("Part 2: {p2}");

    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}
