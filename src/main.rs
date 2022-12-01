#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]

mod util;
mod task01;
mod task02;
// mod task03;
// mod task04;
// mod task05;
// mod task06;
// mod task07;
// mod task08;
// mod task09;
// mod task10;
// mod task11;
// mod task12;
// mod task13;
// mod task14;
// mod task15;
// mod task16;
// mod task17;
// mod task18;
// mod task19;

use task01::task01;
use task02::task02;
// use task03::task03;
// use task04::task04;
// use task05::task05;
// use task06::task06;
// use task07::task07;
// use task08::task08;
// use task09::task09;
// use task10::task10;
// use task11::task11;
// use task12::task12;
// use task13::task13;
// use task14::task14;
// use task15::task15;
// use task16::task16;
// use task17::task17;
// use task18::task18;
// use task19::task19;

// fn run(day: usize, task: impl FnOnce()) -> u128 {
//     println!("Day {}:", day + 1);
//     let start = std::time::Instant::now();
//     task();
//     start.elapsed().as_millis()
// }

// const TASKS: [fn(); 19] = [
//     task01, task02, task03, task04, task05, task06, task07, task08, task09, task10, task11,
//     task12, task13, task14, task15, task16, task17, task18, task19 ];

fn main() {
    task01();
    // let mut timings = [0; TASKS.len()];
    // let start = std::time::Instant::now();
    // for (i, task) in TASKS.iter().enumerate() {
    //     timings[i] = run(i, task);
    // }
    // println!("Total time for all days: {}ms", start.elapsed().as_millis());
    // println!("Timings:");
    // for (i, timing) in timings.iter().enumerate() {
    //     println!("Day {:02}: {}ms", i + 1, timing);
    // }
}