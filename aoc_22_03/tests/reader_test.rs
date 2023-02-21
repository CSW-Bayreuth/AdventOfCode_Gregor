use std::path::Path;

use aoc_22_03::*;

#[test]
fn test_read_knapsack_ignore_empty() {
    assert_eq!(
        Path::new("./../input/aoc_22_03/input.txt").read().len(),
        300,
        "aoc_22_03/input.txt has 300 entries, but a different number was parsed"
    );
}
