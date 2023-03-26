use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn start_app() {
    println!(
        "Start-of-packet marker position: {}",
        calc_sop_marker_pos(read_input(Path::new("./input/aoc_22_06/input.txt")))
    );
    println!(
        "Start-of-message marker position: {}",
        calc_som_marker_pos(read_input(Path::new("./input/aoc_22_06/input.txt")))
    );
}

pub fn calc_sop_marker_pos(in_str: String) -> usize {
    calc_marker_pos(in_str, 4)
}

pub fn calc_som_marker_pos(in_str: String) -> usize {
    calc_marker_pos(in_str, 14)
}

fn calc_marker_pos(in_str: String, marker_len: usize) -> usize {
    let char_vec = in_str.chars().collect::<Vec<char>>();

    // iterate all possible marker start positions
    'marker_pos_loop: for marker_start in 0..in_str.len() {
        let marker_end = marker_start + marker_len;

        // check if any two chars are the same in current marker range
        for c1_pos in marker_start..marker_end {
            for c2_pos in c1_pos + 1..marker_end {
                if char_vec[c1_pos] == char_vec[c2_pos] {
                    continue 'marker_pos_loop;
                }
            }
        }

        return marker_end; // no duplicate was found
    }

    panic!("No marker found");
}

fn _calc_sop_marker_pos_legacy(in_str: String) -> usize {
    itertools::multizip((
        in_str.chars().skip(0),
        in_str.chars().skip(1),
        in_str.chars().skip(2),
        in_str.chars().skip(3),
    ))
    .enumerate()
    .skip_while(|(_, (c1, c2, c3, c4))| {
        c1 == c2 || c1 == c3 || c1 == c4 || c2 == c3 || c2 == c4 || c3 == c4
    })
    .next()
    .map(|(idx, _)| idx + 4)
    .unwrap()
}

pub fn read_input(path: &Path) -> String {
    BufReader::new(File::open(path).expect("Input file does not open"))
        .lines()
        .next()
        .expect("Input file does not contain input")
        .expect("Input file does not contain input")
}
