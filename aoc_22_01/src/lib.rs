pub mod model;
pub mod reader;
pub mod utils;

use model::Elf;
use reader::read;
use std::path::Path;

pub fn start_app() {
    let n_most_calory_elves =
        n_most_calory_elves(read(Path::new("./input/aoc_22_01/input.txt")), 3);

    for (place, (elf_no, calory_sum)) in n_most_calory_elves
        .iter()
        .enumerate()
        .map(|(i, t)| (i + 1, t))
    {
        println!(
            "Elf No. {:?} carries the {:?}-most calories, namely {:?}.",
            elf_no, place, calory_sum
        );
    }

    println!(
        "\nThe sum of the three highest calory sums is {:?}.",
        n_most_calory_elves.iter().map(|(_, num)| num).sum::<u32>()
    );
}

fn n_most_calory_elves(elflist: Vec<Elf>, n: usize) -> Vec<(usize, u32)> {
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
