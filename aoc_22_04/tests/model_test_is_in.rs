// ----------------------------------------------------
// imports
// ----------------------------------------------------
use aoc_22_04::*;

// ----------------------------------------------------
// test funcs
// ----------------------------------------------------
#[test]
fn test_read_id_ranges_equal() {
    assert!(
        IdRange(5, 7).is_in(&IdRange(5, 7)),
        "IdRange(5, 7) is in itself"
    );
}

#[test]
fn test_read_id_ranges_1equal_2bigger() {
    assert!(
        IdRange(5, 7).is_in(&IdRange(5, 8)),
        "IdRange(5, 7) is in IdRange(5, 8)"
    );
}

#[test]
fn test_read_id_ranges_1equal_2smaller() {
    assert!(
        !IdRange(5, 7).is_in(&IdRange(5, 6)),
        "IdRange(5, 7) is not in IdRange(5, 6)"
    );
}

#[test]
fn test_read_id_ranges_1bigger_2equal() {
    assert!(
        !IdRange(5, 7).is_in(&IdRange(6, 7)),
        "IdRange(5, 7) is not in IdRange(6, 7)"
    );
}

#[test]
fn test_read_id_ranges_1smaller_2equal() {
    assert!(
        IdRange(5, 7).is_in(&IdRange(4, 7)),
        "IdRange(5, 7) is in IdRange(4, 7)"
    );
}
