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

use task01::task01;
use task02::task02;

fn main() {
    task01();
}