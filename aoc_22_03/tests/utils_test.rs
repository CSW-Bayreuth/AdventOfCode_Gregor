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
        as_elf_groups(knapsacks).len(),
        knapsacks.len() / 3,
        "aoc_22_03/input.txt: not thrice as many knapsacks as elfs"
    );
}

#[test]
fn test_read_inputsample_knapsacks_vs_elfgroups() {
    let knapsacks = &Path::new(INPUT_SAMPLE_PATH).read();
    assert_eq!(
        as_elf_groups(knapsacks).len(),
        knapsacks.len() / 3,
        "aoc_22_03/input_example.txt: not thrice as many knapsacks as elfs"
    );
}

#[test]
fn test_inputsample_common_item_types() {
    let badge_types = as_elf_groups(&Path::new(INPUT_SAMPLE_PATH).read())
        .iter()
        .map(find_common_item)
        .map(|i| i.0)
        .collect::<Vec<char>>();

    assert_eq!(
        badge_types,
        vec!['r', 'Z'],
        "aoc_22_03/input_example.txt: common item types should be 'r' and 'Z'"
    )
}

#[test]
fn test_inputsample_sum_common_item_priorities() {
    assert_eq!(
        sum_common_item_priorities(&Path::new(INPUT_SAMPLE_PATH).read()),
        70,
        "aoc_22_03/input_example.txt: common item type priority sum should be 70"
    )
}
