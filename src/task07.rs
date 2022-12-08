use std::collections::{HashMap, hash_map::Entry};

const TOTAL_SPACE: usize = 70_000_000;
const NEEDED_SPACE: usize = 30_000_000;

struct Directory {
    parent: usize,
    name: String,
    children: HashMap<String, usize>,
}

enum Node {
    Directory(Directory),
    File { size: usize }
}

impl Node {
    const fn as_directory(&self) -> Option<&Directory> {
        match self {
            Self::Directory(dir) => Some(dir),
            Self::File{ .. } => None
        }
    }

    fn as_directory_mut(&mut self) -> Option<&mut Directory> {
        match self {
            Self::Directory(dir) => Some(dir),
            Self::File{ .. } => None
        }
    }
}

fn sum_directories_under_size(tree: &[Node], root: usize, sizes: &mut Vec<usize>) -> usize {
    let current_dir = &tree[root];
    match current_dir {
        &Node::File { size } => size,
        Node::Directory(dir) => {
            let mut size = 0;
            for (name, &child) in &dir.children {
                if name == "/" {
                    continue;
                }
                size += sum_directories_under_size(tree, child, sizes);
            }
            sizes.push(size);
            size
        }
    }
}

pub fn task07() {
    let start = std::time::Instant::now();
    let text = include_str!("../tasks/task07.txt");

    let mut nodes: Vec<Node> = Vec::new();
    let mut current_dir = 0;
    nodes.push(Node::Directory(Directory {
        parent: 0,
        name: "/".into(),
        children: HashMap::new()
    }));
    nodes[0].as_directory_mut().unwrap().children.insert("/".into(), 0);
    for instruction in text.lines() {
        match instruction {
            i if i.starts_with("$ cd") => {
                let dir = &i[5..];
                if dir == ".." {
                    current_dir = nodes[current_dir].as_directory().unwrap().parent;
                } else {
                    current_dir = *nodes[current_dir].as_directory().unwrap().children.get(dir).unwrap();
                }
            }
            i if i.starts_with("$ ls") => {
                continue;
            }
            ls_output if ls_output.starts_with("dir") => {
                let new_dir_idx = nodes.len();
                let name = ls_output[4..].to_string();
                let new_dir = Directory {
                    parent: current_dir,
                    name: name.clone(),
                    children: HashMap::new()
                };
                nodes.push(Node::Directory(new_dir));
                nodes[current_dir].as_directory_mut().unwrap().children.insert(name, new_dir_idx);
            }
            file => {
                let (size, name) = file.split_once(' ').unwrap();
                let size = size.parse::<usize>().unwrap();
                let name = name.to_string();
                let new_file_idx = nodes.len();
                nodes.push(Node::File { size });
                nodes[current_dir].as_directory_mut().unwrap().children.insert(name, new_file_idx);
            }
        }
    }

    let mut sizes = Vec::new();
    let total_used = sum_directories_under_size(&nodes, 0, &mut sizes);

    println!("Part 1: {}", sizes.iter().filter(|&&ds| ds <= 100_000).sum::<usize>());
    let space_left = TOTAL_SPACE - total_used;
    let at_least_must_be_freed = NEEDED_SPACE - space_left;
    let choice = sizes.iter().filter(|&&s| s >= at_least_must_be_freed).min().unwrap();
    println!("Part 2: {choice}");

    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}