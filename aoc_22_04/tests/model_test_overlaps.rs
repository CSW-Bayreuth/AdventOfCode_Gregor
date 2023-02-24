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
        IdRange(5, 7).overlaps(&IdRange(5, 7)),
        "IdRange(5, 7) overlaps itself"
    );
}

#[test]
fn test_read_id_ranges_1equal_2bigger() {
    assert!(
        IdRange(5, 7).overlaps(&IdRange(5, 8)),
        "IdRange(5, 7) overlaps IdRange(5, 8)"
    );
}

#[test]
fn test_read_id_ranges_1equal_2smaller() {
    assert!(
        IdRange(5, 7).overlaps(&IdRange(5, 6)),
        "IdRange(5, 7) overlaps IdRange(5, 6)"
    );
}

#[test]
fn test_read_id_ranges_1bigger_2equal() {
    assert!(
        IdRange(5, 7).overlaps(&IdRange(6, 7)),
        "IdRange(5, 7) overlaps IdRange(6, 7)"
    );
}

#[test]
fn test_read_id_ranges_1smaller_2equal() {
    assert!(
        IdRange(5, 7).overlaps(&IdRange(4, 7)),
        "IdRange(5, 7) overlaps IdRange(4, 7)"
    );
}

#[test]
fn test_read_id_ranges_apart_left_1() {
    assert!(
        !IdRange(13, 14).overlaps(&IdRange(8, 12)),
        "IdRange(13, 14) overlaps not IdRange(8, 12)"
    );
}

#[test]
fn test_read_id_ranges_overlap_left_1() {
    assert!(
        IdRange(12, 14).overlaps(&IdRange(7, 12)),
        "IdRange(12, 14) overlaps IdRange(7, 12)"
    );
}

#[test]
fn test_read_id_ranges_overlap_right_1() {
    assert!(
        IdRange(5, 7).overlaps(&IdRange(7, 12)),
        "IdRange(5, 7) overlaps IdRange(7, 12)"
    );
}

#[test]
fn test_read_id_ranges_apart_right_1() {
    assert!(
        !IdRange(5, 7).overlaps(&IdRange(8, 12)),
        "IdRange(5, 7) overlaps not IdRange(8, 12)"
    );
}
