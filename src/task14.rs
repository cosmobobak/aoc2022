#![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::similar_names, unused_variables)]

const SOURCE_COORD: (i64, i64) = (500, 0);
const BLOCKER: u8 = b'#';
const SOURCE: u8 = b'+';
const EMPTY: u8 = b'.';
const SAND: u8 = b'o';

pub fn task14() {
    let start = std::time::Instant::now();
    let input = include_str!("../tasks/task14.txt");

    let (mut max_x, mut min_x) = (i64::MIN, i64::MAX);
    let (mut max_y, mut min_y) = (i64::MIN, i64::MAX);

    for line in input.lines() {
        for coord in line.split(" -> ").map(parse_coord).chain(std::iter::once(SOURCE_COORD)) {
            if coord.0 > max_x {
                max_x = coord.0;
            }
            if coord.0 < min_x {
                min_x = coord.0;
            }
            if coord.1 > max_y {
                max_y = coord.1;
            }
            if coord.1 < min_y {
                min_y = coord.1;
            }
        }
    }

    max_y += 1;
    min_y -= 1;
    let height_above_source = max_y - SOURCE_COORD.1;
    let left_dist_from_source = SOURCE_COORD.0 - min_x;
    let right_dist_from_source = max_x - SOURCE_COORD.0;
    min_x -= height_above_source - left_dist_from_source + 1;
    max_x += height_above_source - right_dist_from_source + 1;

    let dims = (max_x - min_x + 1, max_y - min_y + 1);
    let normalise_x = |x: i64| x - min_x;
    let normalise_y = |y: i64| y - min_y;
    let denormalise_x = |x: i64| x + min_x;
    let denormalise_y = |y: i64| y + min_y;

    let mut map = vec![EMPTY; (dims.0 * dims.1) as usize];

    for line in input.lines() {
        let mut coords = line.split(" -> ");
        let source = parse_coord(coords.next().unwrap());
        let mut source = (normalise_x(source.0), normalise_y(source.1));
        for coord in coords {
            let target = parse_coord(coord);
            let target = (normalise_x(target.0), normalise_y(target.1));
            let (from_x, to_x) = (source.0.min(target.0), source.0.max(target.0));
            let (from_y, to_y) = (source.1.min(target.1), source.1.max(target.1));
            if from_x == to_x {
                let x = source.0;
                for y in from_y..=to_y {
                    map[(y * dims.0 + x) as usize] = BLOCKER;
                }
            } else {
                let y = source.1;
                for x in from_x..=to_x {
                    map[(y * dims.0 + x) as usize] = BLOCKER;
                }
            }
            source = target;
        }
    }

    let mut iterations = 0;

    while drop_sand::<true>(&mut map, dims, &normalise_x, &normalise_y) {
        iterations += 1;
    }

    println!("Part 1: {iterations}");

    while drop_sand::<false>(&mut map, dims, &normalise_x, &normalise_y) {
        iterations += 1;
    }

    println!("Part 2: {iterations}");

    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}

fn parse_coord(coord: &str) -> (i64, i64) {
    coord.split_once(',').map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())).unwrap()
}

fn print_map(map: &[u8], dims: (i64, i64), denorm_x: &dyn Fn(i64) -> i64, denorm_y: &dyn Fn(i64) -> i64) {
    for y in 0..dims.1 {
        for x in 0..dims.0 {
            if (denorm_x(x), denorm_y(y)) == SOURCE_COORD {
                print!("{}", SOURCE as char);
                continue;
            }
            print!("{}", map[(y * dims.0 + x) as usize] as char);
        }
        println!();
    }
    println!();
}

fn drop_sand<const PART_1: bool>(map: &mut [u8], (xdim, ydim): (i64, i64), norm_x: &dyn Fn(i64) -> i64, norm_y: &dyn Fn(i64) -> i64) -> bool {
    let mut sand_pos = SOURCE_COORD;
    sand_pos.0 = norm_x(sand_pos.0);
    sand_pos.1 = norm_y(sand_pos.1);
    if map[(sand_pos.1 * xdim + sand_pos.0) as usize] == SAND {
        return false;
    }
    loop {
        if sand_pos.1 == ydim - 1 {
            if PART_1 {
                break false;
            }
            map[(sand_pos.1 * xdim + sand_pos.0) as usize] = SAND;
            break true;
        }
        if map[((sand_pos.1 + 1) * xdim + sand_pos.0) as usize] == EMPTY {
            sand_pos.1 += 1;
            continue;
        }
        if map[((sand_pos.1 + 1) * xdim + (sand_pos.0 - 1)) as usize] == EMPTY {
            sand_pos.1 += 1;
            sand_pos.0 -= 1;
            continue;
        }
        if map[((sand_pos.1 + 1) * xdim + (sand_pos.0 + 1)) as usize] == EMPTY {
            sand_pos.1 += 1;
            sand_pos.0 += 1;
            continue;
        }
        map[(sand_pos.1 * xdim + sand_pos.0) as usize] = SAND;
        break true;
    }
}