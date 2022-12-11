use crate::util::borrow_indexes;

#[derive(Clone, Copy)]
pub struct StaticStack<T: Copy, const CAPACITY: usize> {
    stack: [T; CAPACITY],
    top: usize,
}

impl<const CAPACITY: usize> StaticStack<u8, CAPACITY> {
    const fn new() -> Self {
        Self {
            stack: [0; CAPACITY],
            top: 0,
        }
    }

    fn push(&mut self, value: u8) {
        if self.top < CAPACITY {
            self.stack[self.top] = value;
            self.top += 1;
        }
    }

    fn pop(&mut self) -> Option<u8> {
        if self.top > 0 {
            self.top -= 1;
            Some(self.stack[self.top])
        } else {
            None
        }
    }

    fn data_mut(&mut self) -> &mut [u8] {
        &mut self.stack[..self.top]
    }

    const fn peek(&self) -> Option<u8> {
        if self.top > 0 {
            Some(self.stack[self.top - 1])
        } else {
            None
        }
    }

    const fn len(&self) -> usize {
        self.top
    }

    fn add_slice(&mut self, slice: &[u8]) {
        for &value in slice {
            self.push(value);
        }
    }

    fn drop_last(&mut self, n: usize) {
        self.top = self.top.saturating_sub(n);
    }
}

pub fn task05() {
    let start = std::time::Instant::now();
    let (arrangement, instructions) = include_str!("../tasks/task05.txt")
        .split_once("\r\n\r\n")
        .unwrap();

    let mut stacks = [StaticStack::<u8, 100>::new(); 9];
    for line in arrangement.lines().rev().skip(1) {
        let bytes = line.as_bytes();
        for (j, stack) in stacks.iter_mut().enumerate() {
            let value = bytes[j * 4 + 1];
            if value != b' ' {
                stack.push(value);
            }
        }
    }

    let parse_inst = |inst: &str| {
        let mut parts = inst
            .split_whitespace()
            .filter_map(|p| p.parse::<usize>().ok());
        let n = parts.next().unwrap();
        let from = parts.next().unwrap() - 1;
        let to = parts.next().unwrap() - 1;
        (from, to, n)
    };

    let stacks_part_2 = stacks;

    for instruction in instructions.lines() {
        let (from, to, n) = parse_inst(instruction);
        for _ in 0..n {
            let val = stacks[from].pop().unwrap();
            stacks[to].push(val);
        }
    }

    print!("Part 1: ");
    for s in &stacks {
        print!("{}", s.peek().unwrap() as char);
    }
    println!();

    let mut stacks = stacks_part_2;
    for instruction in instructions.lines() {
        let (from, to, n) = parse_inst(instruction);
        let (from_stack, to_stack) = borrow_indexes(&mut stacks, from, to);
        let from_len = from_stack.len();
        let slice_to_read = &mut from_stack.data_mut()[from_len - n..];
        to_stack.add_slice(slice_to_read);
        from_stack.drop_last(n);
    }

    print!("Part 2: ");
    for s in &stacks {
        print!("{}", s.peek().unwrap() as char);
    }
    println!();

    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}
