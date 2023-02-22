use std::path::Path;

// ----------------------------------------------------
// imports
// ----------------------------------------------------
use aoc_22_03::*;

// ----------------------------------------------------
// tests constants
// ----------------------------------------------------
const INPUT_PROPER_PATH: &str = "./../input/aoc_22_03/input.txt";
const INPUT_SAMPLE_PATH: &str = "./../input/aoc_22_03/input_example.txt";

// ----------------------------------------------------
// test funcs
// ----------------------------------------------------
#[test]
fn test_read_inputproper_knapsacks_vs_elfgroups() {
    let knapsacks = &Path::new(INPUT_PROPER_PATH).read();
    assert_eq!(
        calc_elf_groups(knapsacks).len(),
        knapsacks.len() / 3,
        "aoc_22_03/input.txt: not thrice as many knapsacks as elfs"
    );
}

#[test]
fn test_read_inputsample_knapsacks_vs_elfgroups() {
    let knapsacks = &Path::new(INPUT_SAMPLE_PATH).read();
    assert_eq!(
        calc_elf_groups(knapsacks).len(),
        knapsacks.len() / 3,
        "aoc_22_03/input_example.txt: not thrice as many knapsacks as elfs"
    );
}
