// ----------------------------------------------------
// modules
// ----------------------------------------------------
pub mod filesystem;
pub mod terminal_output;

// ----------------------------------------------------
// imports
// ----------------------------------------------------
pub use filesystem::*;
pub use terminal_output::*;

// ----------------------------------------------------
// App
// ----------------------------------------------------
pub fn start_app() {
    let dirtree = parse_terminal_output_to_dirtree("./input/aoc_22_07/input.txt");

    println!(
        "Sum of dir sizes of dirs with size lower 100000: {}",
        sum_sizes_of_dirs_with_size_lower_max(&dirtree, 100000)
    )
}
