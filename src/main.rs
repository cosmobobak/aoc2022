#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]
#![allow(unused_imports, dead_code)]

mod util;
mod task01;
mod task02;
mod task03;
mod task04;
mod task05;
mod task06;
mod task07;
mod task7test;

use task01::task01;
use task02::task02;
use task03::task03;
use task04::task04;
use task05::task05;
use task06::task06;
use task07::task07;

fn main() {
    task07();
}