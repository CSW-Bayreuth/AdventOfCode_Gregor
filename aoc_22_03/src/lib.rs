mod model;
mod reader;
mod utils;

use std::path::Path;

pub use model::{Compartment, Item, Knapsack};
pub use reader::Reader;

pub fn start_app() {
    let _knapsacks = Path::new("./input/aoc_22_03/input.txt").read();
    println!("Hello aoc_22_03!");
}
