#![allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]

use std::fmt::Display;

use graphsearch::{graph::{Graph, WeightedGraph}, graphsearcher::GraphSearcher};

const CONTROL_GREEN: &str = "\u{001b}[32m";
const CONTROL_RED: &str = "\u{001b}[31m";
const CONTROL_RESET: &str = "\u{001b}[0m";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coordinate(usize, usize);

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

struct HeightMap {
    data: Vec<Vec<u8>>,
    src_criterion: fn(u8) -> bool,
    tgt_criterion: fn(u8) -> bool,
    mov_criterion: fn(u8, u8) -> bool,
}

impl HeightMap {
    fn at(&self, c: Coordinate) -> u8 {
        self.data[c.0][c.1]
    }
}

const fn elev(x: u8) -> u8 {
    match x {
        b'S' => b'a',
        b'E' => b'z',
        _ => x,
    }
}

impl Graph for HeightMap {
    type Node = Coordinate;
    type Edge = usize;

    fn root(&self) -> Self::Node {
        for row in 0..self.data.len() {
            for col in 0..self.data[0].len() {
                let val = self.at(Coordinate(row, col));
                if (self.src_criterion)(val) {
                    return Coordinate(row, col);
                }
            }
        }
        unreachable!();
    }

    fn children(&self, node: Self::Node) -> Vec<Self::Node> {
        let mut out = Vec::with_capacity(4);
        let value_here = elev(self.data[node.0][node.1]);
        for dir in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new = (node.0 as isize + dir.0, node.1 as isize + dir.1);
            if new.0 >= 0 && new.1 >= 0 && new.0 < self.data.len() as isize && new.1 < self.data[0].len() as isize {
                let value_at = elev(self.data[new.0 as usize][new.1 as usize]);
                if !(self.mov_criterion)(value_here, value_at) {
                    continue;
                }
                out.push(Coordinate(new.0 as usize, new.1 as usize));
            }
        }
        out
    }

    fn edges(&self, _node: Self::Node) -> Vec<Self::Edge> {
        unimplemented!()
    }

    fn is_goal(&self, node: Self::Node) -> bool {
        (self.tgt_criterion)(self.at(node))
    }
}

impl WeightedGraph for HeightMap {
    fn edge_weight(&self, _from: Self::Node, _to: Self::Node) -> i64 {
        1
    }
}

pub fn task12() {
    let start = std::time::Instant::now();
    let data = include_str!("../tasks/task12.txt");

    let mut map = HeightMap {
        data: Vec::new(),
        src_criterion: |loc| loc == b'S',
        tgt_criterion: |loc| loc == b'E',
        mov_criterion: |from, to| to - 1 <= from,
    };

    for line in data.lines() {
        map.data.push(line.as_bytes().to_vec());
    }

    let mut dijk = graphsearch::dijkstra::Dijkstra::new();
    let _ = dijk.search_tracked(&map, map.root()).unwrap();
    let path = dijk.path().unwrap();
    let steps = path.len() - 1;

    println!("Part 1: {steps}");

    map.src_criterion = |loc| elev(loc) == b'a';
    map.mov_criterion = |from, to| from - 1 <= to;
    std::mem::swap(&mut map.src_criterion, &mut map.tgt_criterion);
    let _ = dijk.search_tracked(&map, map.root()).unwrap();
    let path = dijk.path().unwrap();
    let steps = path.len() - 1;

    println!("Part 2: {steps}");

    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}

fn render_path(map: &HeightMap, path: &[Coordinate]) {
    for row in 0..map.data.len() {
        for col in 0..map.data[0].len() {
            let c = Coordinate(row, col);
            if path.contains(&c) {
                print!("{CONTROL_RED}@{CONTROL_RESET}");
            } else {
                print!("{CONTROL_GREEN}{}{CONTROL_RESET}", map.at(c) as char);
            }
        }
        println!();
    }
    println!();
}