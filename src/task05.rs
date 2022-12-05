

pub fn task05() {
    let (arrangement, instructions) = include_str!("../tasks/task05.txt").split_once("\r\n\r\n").unwrap();

    let mut heights = [0usize; 9];
    let mut stacks = [[0; 100]; 9];
    for (i, line) in arrangement.lines().rev().skip(1).enumerate() {
        let bytes = line.as_bytes();
        for (j, h) in heights.iter_mut().enumerate() {
            let value = bytes[j * 4 + 1];
            stacks[j][i] = value;
            if value != b' ' {
                *h = i + 1;
            }
        }
    }

    let heights_part_2 = heights;
    let stacks_part_2 = stacks;
    
    let mut make_move = |from: usize, to: usize| {
        let fh = heights[from];
        let th = heights[to];
        let value = std::mem::replace(&mut stacks[from][fh - 1], b' ');
        stacks[to][th] = value;
        heights[from] -= 1;
        heights[to] += 1;
    };

    for instruction in instructions.lines() {
        let mut parts = instruction.split_whitespace().filter_map(|p| p.parse::<usize>().ok());
        let n = parts.next().unwrap();
        let from = parts.next().unwrap() - 1;
        let to = parts.next().unwrap() - 1;
        for _ in 0..n {
            make_move(from, to);
        }
    }

    print!("Part 1: ");
    for (i, &h) in heights.iter().enumerate() {
        print!("{}", stacks[i][h - 1] as char);
    }
    println!();

    let mut heights = heights_part_2;
    let mut stacks = stacks_part_2;
    let mut make_move_9001 = |from: usize, to: usize, n: usize| {
        let (from_stack, to_stack) = if from < to {
            let split = stacks.split_at_mut(to);
            (&mut split.0[from], &mut split.1[0]) 
        } else {
            let split = stacks.split_at_mut(from);
            (&mut split.1[0], &mut split.0[to]) 
        };
        let slice_to_read = &mut from_stack[heights[from] - n..heights[from]];
        let slice_to_write = &mut to_stack[heights[to]..heights[to] + n];
        slice_to_write.copy_from_slice(slice_to_read);
        slice_to_read.fill(b' ');
        heights[from] -= n;
        heights[to] += n;
    };

    for instruction in instructions.lines() {
        let mut parts = instruction.split_whitespace().filter_map(|p| p.parse::<usize>().ok());
        let n = parts.next().unwrap();
        let from = parts.next().unwrap() - 1;
        let to = parts.next().unwrap() - 1;
        make_move_9001(from, to, n);
    }

    print!("Part 2: ");
    for (i, &h) in heights.iter().enumerate() {
        print!("{}", stacks[i][h - 1] as char);
    }
    println!();
}