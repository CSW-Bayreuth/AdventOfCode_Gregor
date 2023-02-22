// ----------------------------------------------------
// imports
// ----------------------------------------------------
use crate::{Item, Knapsack};

// ----------------------------------------------------
// utils: wrong items and summed priorities
// ----------------------------------------------------
pub fn calc_sum_of_wrong_item_priorities(knapsacks: Vec<Knapsack>) -> usize {
    knapsacks
        .iter()
        .map(find_wrong_items)
        .map(Option::unwrap)
        .map(Item::priority)
        .sum()
}

pub fn find_wrong_items(knapsack: &Knapsack) -> Option<&Item> {
    for ref item in &knapsack.cp1 {
        if knapsack.cp2.contains(&item) {
            return Some(item);
        }
    }
    None
}
