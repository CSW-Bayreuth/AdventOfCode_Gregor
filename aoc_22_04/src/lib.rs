// ----------------------------------------------------
// modules
// ----------------------------------------------------
mod model;
mod reader;
mod utils;

// ----------------------------------------------------
// imports and re-exports
// ----------------------------------------------------
use std::path::Path;

pub use crate::model::IdRange;
pub use crate::reader::Reader;
pub use crate::utils::id_range_pairs_overlapping;
pub use crate::utils::id_range_pairs_with_one_containing;

// ----------------------------------------------------

// ----------------------------------------------------
// starter func
// ----------------------------------------------------
pub fn start_app() {
    let id_range_pairs = Path::new("./input/aoc_22_04/input.txt").read();

    println!(
        "Number of id range pairs which fully contain each other: {}",
        id_range_pairs_with_one_containing(&id_range_pairs)
    );

    println!(
        "Number of id range pairs overlapping: {}",
        id_range_pairs_overlapping(&id_range_pairs)
    );
}
