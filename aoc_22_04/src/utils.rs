// ----------------------------------------------------
// imports
// ----------------------------------------------------
use crate::model::IdRange;

// ----------------------------------------------------
// part 1: id range pairs with one containing the other
// ----------------------------------------------------
pub fn id_range_pairs_with_one_containing(id_range_pairs: &Vec<(IdRange, IdRange)>) -> usize {
    id_range_pairs
        .iter()
        .map(|(r1, r2)| (r1.is_in(r2) || r2.is_in(r1)))
        .map(|b| if b { 1 } else { 0 })
        .sum()
}

// ----------------------------------------------------
// part 2: id range pairs that overlap
// ----------------------------------------------------
pub fn id_range_pairs_overlapping(id_range_pairs: &Vec<(IdRange, IdRange)>) -> usize {
    id_range_pairs
        .iter()
        .map(|(r1, r2)| r1.overlaps(r2))
        .map(|b| if b { 1 } else { 0 })
        .sum()
}
