use std::path::Path;

mod model;
mod reader;

pub fn start_app() {
    let guide = reader::read_strategy_guides(Path::new("./input/aoc_22_02/input.txt"));
}
