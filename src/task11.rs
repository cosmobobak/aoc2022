#![allow(clippy::cast_sign_loss)]

use crate::util::MinMaxN;

#[derive(Clone, Copy)]
enum Op { Add(i64), Mul(i64), Square }

impl Op {
    fn apply(&self, old: i64) -> i64 {
        match self {
            Self::Add(n) => old + n,
            Self::Mul(n) => old * n,
            Self::Square => old * old,
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    op: Op,
    test: i64,
    to: [usize; 2],
    inspections: u64,
}

fn turn<const IS_PART_1: bool>(ms: &mut [Monkey], m: usize, global_modulus: i64) {
    let items = std::mem::take(&mut ms[m].items);
    for worry_level in items {
        // monkey inspects the item
        let new = ms[m].op.apply(worry_level);
        ms[m].inspections += 1;
        // you become less worried
        let new = if IS_PART_1 { new / 3 } else { new % global_modulus };
        // monkey performs the worry test
        let is_divisible = new % ms[m].test == 0;
        // based on this, monkey decides which monkey to give the item to
        let to = ms[m].to[usize::from(is_divisible)];
        // monkey gives the item to the other monkey
        ms[to].items.push(new);
    }
}

pub fn task11() {
    let start = std::time::Instant::now();
    let data = include_str!("../tasks/task11.txt");

    let mut monkeys = Vec::new();
    for monkey_block in data.split("\r\n\r\n") {
        let mut lines = monkey_block.lines();
        lines.next();
        let starting_items: Vec<i64> = lines.next().unwrap().split_once(": ").unwrap().1.split(", ").map(|s| s.parse().unwrap()).collect();
        let op = lines.next().unwrap().split_once("new = old ").unwrap().1;
        let op = match op {
            "* old" => Op::Square,
            op if op.starts_with("+ ") => Op::Add(op[2..].parse().unwrap()),
            op if op.starts_with("* ") => Op::Mul(op[2..].parse().unwrap()),
            _ => panic!("Unknown op: {}", op),
        };
        let test = lines.next().unwrap().split_once("divisible by ").unwrap().1.parse().unwrap();
        let to1: usize = lines.next().unwrap().split_once("to monkey ").unwrap().1.parse().unwrap();
        let to2: usize = lines.next().unwrap().split_once("to monkey ").unwrap().1.parse().unwrap();
        monkeys.push(Monkey {
            items: starting_items,
            op,
            test,
            to: [to2, to1], // swapped because true comes first
            inspections: 0,
        });
    }

    let saved_initial_monkey_state = monkeys.clone();
    for _ in 0..20 {
        for m in 0..monkeys.len() {
            turn::<true>(&mut monkeys, m, 0);
        }
    }
    let two_most_active = monkeys.iter().map(|m| u128::from(m.inspections)).max_n_ct::<2>().unwrap();

    println!("Part 1: {}", two_most_active[0] * two_most_active[1]);

    let mut monkeys = saved_initial_monkey_state;
    let global_modulus = monkeys.iter().map(|m| m.test).product::<i64>();
    for _ in 0..10_000 {
        for m in 0..monkeys.len() {
            turn::<false>(&mut monkeys, m, global_modulus);
        }
    }
    let two_most_active = monkeys.iter().map(|m| u128::from(m.inspections)).max_n_ct::<2>().unwrap();

    println!("Part 2: {}", two_most_active[0] * two_most_active[1]);

    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}