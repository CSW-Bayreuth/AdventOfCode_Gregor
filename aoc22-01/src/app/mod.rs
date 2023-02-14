pub mod model;
pub mod reader;
pub mod utils;

use crate::app::model::Elf;
use crate::app::reader::read_elfs_and_foodrations;
use std::path::Path;

pub fn start_app() {
    let mut elflist: Vec<Elf> = Vec::new();

    read_elfs_and_foodrations(Path::new("./input.txt"), &mut elflist);

    let n_most_calory_elves = find_n_elfs_with_most_calories(&elflist, 3);

    for (i, (elf_no, calory_sum)) in n_most_calory_elves.iter().enumerate() {
        println!(
            "Elf No. {:?} carries the {:?}-most calories, namely {:?}.",
            elf_no + 1,
            i + 1,
            calory_sum
        );
    }

    println!("");

    println!(
        "The sum of the three highest calory sums is {:?}.",
        n_most_calory_elves
            .iter()
            .map(|(_, cal_sum)| cal_sum)
            .sum::<u32>()
    );
}

fn find_n_elfs_with_most_calories(elflist: &Vec<Elf>, n: usize) -> Vec<(usize, u32)> {
    let mut index_sum_list = elflist
        .iter()
        .enumerate()
        .map(|(index, elf)| {
            (
                index,
                elf.inventory
                    .iter()
                    .map(|ration| ration.calories)
                    .sum::<u32>(),
            )
        })
        .collect::<Vec<(usize, u32)>>();

    index_sum_list.sort_by(|(_, cal1), (_, cal2)| cal2.partial_cmp(cal1).unwrap());

    index_sum_list
        .iter()
        .take(n)
        .map(|t| t.clone())
        .collect::<Vec<(usize, u32)>>()
}
