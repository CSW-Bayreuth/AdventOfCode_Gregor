// ----------------------------------------------------
// imports
// ----------------------------------------------------
use crate::{model::ElfGroup, Item, Knapsack};

// ----------------------------------------------------
// part 1: wrong items and summed priorities
// ----------------------------------------------------
pub fn sum_of_wrong_item_priorities(knapsacks: &Vec<Knapsack>) -> usize {
    knapsacks
        .iter()
        .map(find_wrong_item)
        .map(Item::priority)
        .sum()
}

pub fn find_wrong_item(knapsack: &Knapsack) -> &Item {
    knapsack
        .cp1
        .iter()
        .find(|ref item| knapsack.cp2.contains(item))
        .unwrap()
}

// ----------------------------------------------------
// part 2: elf groups, badge finding, priority
// ----------------------------------------------------
pub fn calc_elf_groups<'a>(knapsacks: &'a Vec<Knapsack>) -> Vec<ElfGroup> {
    knapsacks
        .iter()
        .enumerate()
        .map(|t| t.0)
        .fold(Vec::new(), |mut groups, i| {
            if i % 3 == 2 {
                groups.push((&knapsacks[i - 2], &knapsacks[i - 1], &knapsacks[i]));
            }
            groups
        })
}
