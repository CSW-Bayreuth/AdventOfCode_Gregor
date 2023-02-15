pub mod model;
pub mod reader;
pub mod utils;

use model::Elf;
use reader::read_elfs_and_foodrations;
use std::path::Path;

pub fn start_app() {
    let mut elflist: Vec<Elf> = Vec::new();

    read_elfs_and_foodrations(Path::new("./input/aoc_22_01/input.txt"), &mut elflist);

    let n_most_calory_elves = n_elfs_with_most_calories(&elflist, 3);

    for (i, (elf_no, calory_sum)) in n_most_calory_elves.iter().enumerate() {
        println!(
            "Elf No. {:?} carries the {:?}-most calories, namely {:?}.",
            elf_no,
            i + 1,
            calory_sum
        );
    }

    println!("");

    println!(
        "The sum of the three highest calory sums is {:?}.",
        sum_element_two(&n_most_calory_elves)
    );
}

fn sum_element_two(vec_of_pairs: &Vec<(usize, u32)>) -> u32 {
    vec_of_pairs.iter().map(|(_, num)| num).sum::<u32>()
}

fn n_elfs_with_most_calories(elflist: &Vec<Elf>, n: usize) -> Vec<(usize, u32)> {
    let mut index_sum_list = elflist
        .iter()
        .enumerate()
        .map(|(index, elf)| {
            (
                index + 1,
                elf.inventory
                    .iter()
                    .map(|ration| ration.calories)
                    .sum::<u32>(),
            )
        })
        .collect::<Vec<(usize, u32)>>(); // elf_no, calory_sum

    index_sum_list.sort_by(|(_, cal1), (_, cal2)| cal2.partial_cmp(cal1).unwrap());
    index_sum_list.truncate(n);

    index_sum_list
}
