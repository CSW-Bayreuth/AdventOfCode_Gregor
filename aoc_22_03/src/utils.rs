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
pub fn as_elf_groups(knapsacks: &'_ Vec<Knapsack>) -> Vec<ElfGroup<'_>> {
    knapsacks
        .iter()
        .enumerate()
        .map(|t| t.0)
        .fold(Vec::new(), |mut groups, i| {
            if i % 3 == 2 {
                groups.push(ElfGroup(
                    &knapsacks[i - 2],
                    &knapsacks[i - 1],
                    &knapsacks[i],
                ));
            }
            groups
        })
}

pub fn find_common_item(elf_group: &ElfGroup) -> Item {
    for c in ('a'..='z').chain('A'..='Z') {
        if elf_group.iter().all(|k| k.items().iter().any(|i| i.0 == c)) {
            return Item(c);
        }
    }
    Item('0')
}

pub fn sum_common_item_priorities<'a>(knapsacks: &Vec<Knapsack>) -> usize {
    as_elf_groups(knapsacks)
        .iter()
        .map(find_common_item)
        .map(|i| i.priority())
        .sum()
}
