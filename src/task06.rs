

pub fn task06() {
    let start = std::time::Instant::now();
    let text = include_str!("../tasks/task06.txt");

    let bytes = text.as_bytes();

    let n = first_window_of_n(bytes, 4);

    println!("Part 1: {n}");

    let n = first_window_of_n(bytes, 14);

    println!("Part 2: {n}");

    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}

fn first_window_of_n(bytes: &[u8], window: usize) -> usize {
    let mut n = None;
    let mut bitvector: u64 = bytes.iter().take(window).fold(0, |acc, &b| acc ^ (1 << (b - b'a')));
    for ((i, &new_byte), &old_byte) in bytes.iter().enumerate().skip(window).zip(bytes.iter()) {
        bitvector ^= 1 << (new_byte - b'a');
        bitvector ^= 1 << (old_byte - b'a');
        let bits = bitvector.count_ones();
        if bits as usize == window {
            n = Some(i + 1);
            break;
        }
    }
    n.unwrap()
}