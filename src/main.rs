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

use task01::task01;
use task02::task02;
use task03::task03;
use task04::task04;

fn main() {
    task04();
}