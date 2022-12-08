
pub fn scenic_score(trees: &[u8], height: usize, width: usize, x: usize, y: usize) -> u32 {
    let index = |x: usize, y: usize| -> usize { y * width + x };
    let mut scores = [0; 4];
    let our_height = u32::from(trees[index(x, y)]);
    // run until there is a tree taller than us, or we reach the end
    // down
    for row in y+1..height {
        let current = u32::from(trees[index(x, row)]);
        scores[0] += 1;
        if current >= our_height {
            break;
        }
    }
    // up
    for row in (0..y).rev() {
        let current = u32::from(trees[index(x, row)]);
        scores[1] += 1;
        if current >= our_height {
            break;
        }
    }
    // right
    for col in x+1..width {
        let current = u32::from(trees[index(col, y)]);
        scores[2] += 1;
        if current >= our_height {
            break;
        }
    }
    // left
    for col in (0..x).rev() {
        let current = u32::from(trees[index(col, y)]);
        scores[3] += 1;
        if current >= our_height {
            break;
        }
    }
    // done
    let score = scores.iter().product();
    score
}

pub fn task08() {
    const LEFT_MASK: u8 = 0b0000_0001;
    const RIGHT_MASK: u8 = 0b0000_0010;
    const UP_MASK: u8 = 0b0000_0100;
    const DOWN_MASK: u8 = 0b0000_1000;
    let start = std::time::Instant::now();
    let text = include_str!("../tasks/task08.txt");
    let mut data = Vec::new();
    let mut width = 0;
    for line in text.lines() {
        data.extend_from_slice(line.as_bytes());
        width = line.len();
    }
    data.iter_mut().for_each(|b| *b -= b'0');
    let height = data.len() / width;
    assert_eq!(data.len(), width * height);
    let index = |x: usize, y: usize| -> usize { y * width + x };
    let mut visible = vec![0; data.len()];
    for row in 0..height {
        let mut highest = -1;
        for col in 0..width {
            let idx = index(col, row);
            let current = data[idx];
            if i32::from(current) > highest {
                visible[idx] |= LEFT_MASK;
            }
            highest = highest.max(i32::from(current));
        }
    }
    for row in 0..height {
        let mut highest = -1;
        for col in (0..width).rev() {
            let idx = index(col, row);
            let current = data[idx];
            if i32::from(current) > highest {
                visible[idx] |= RIGHT_MASK;
            }
            highest = highest.max(i32::from(current));
        }
    }
    for col in 0..width {
        let mut highest = -1;
        for row in 0..height {
            let idx = index(col, row);
            let current = data[idx];
            if i32::from(current) > highest {
                visible[idx] |= UP_MASK;
            }
            highest = highest.max(i32::from(current));
        }
    }
    for col in 0..width {
        let mut highest = -1;
        for row in (0..height).rev() {
            let idx = index(col, row);
            let current = data[idx];
            if i32::from(current) > highest {
                visible[idx] |= DOWN_MASK;
            }
            highest = highest.max(i32::from(current));
        }
    }
    let visible_trees = visible.iter().filter(|&&v| v != 0).count();
    // let mask_sum_chars: [char; 5] = [' ', '.', '*', '&', '#'];
    // for row in 0..height {
    //     for col in 0..width {
    //         let current_idx = index(col, row);
    //         let mask = visible[current_idx];
    //         let mask_sum = mask.count_ones();
    //         let mask_sum_char = mask_sum_chars[mask_sum as usize];
    //         print!(" {}", mask_sum_char);
    //     }
    //     println!();
    // }
    println!("Part 1: {}", visible_trees);

    let best_position_score = (1..width - 1)
        .flat_map(|x| (1..height - 1).map(move |y| (x, y)))
        .map(|(x, y)| (scenic_score(&data, height, width, x, y), x, y))
        .max()
        .unwrap();

    println!("Part 2: {} ({}, {})", best_position_score.0, best_position_score.1, best_position_score.2);

    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}