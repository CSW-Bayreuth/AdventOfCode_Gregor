// ----------------------------------------------------
// imports
// ----------------------------------------------------
use crate::model::IdRange;

// ----------------------------------------------------
// part 1: id range pairs with one containing the other
// ----------------------------------------------------
pub fn id_range_pairs_with_one_containing(idrange_pairs: &Vec<(IdRange, IdRange)>) -> usize {
    idrange_pairs
        .iter()
        .map(|(r1, r2)| (r1.is_in(r2) || r2.is_in(r1)))
        .map(|b| if b { 1 } else { 0 })
        .sum()
}
