// ----------------------------------------------------
// ----------------------------------------------------
// imports
// ----------------------------------------------------
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path
};
// ----------------------------------------------------
// App
// ----------------------------------------------------
pub fn start_app() 
{
    println!(
        "Sum of right extrapolated values: {}",
        sum_of_predicted_values(read_histories(Path::new("./input/aoc_23_09/input.txt")), predict_next_value)
    );
 
    println!(
        "Sum of left extrapolated values: {}",
        sum_of_predicted_values(read_histories(Path::new("./input/aoc_23_09/input.txt")), predict_previous_value)
    );
}

// ----------------------------------------------------
// Evaluation
// ----------------------------------------------------

pub fn sum_of_predicted_values(histories: Vec<History>, predict_func: fn(Vec<Vec<isize>>) -> isize) -> isize
{
    histories.iter().map(History::generate_sequences).map(predict_func).sum()
}

pub fn predict_previous_value(seqs: Vec<Vec<isize>>) -> isize
{
    let mut predicted_new_col = vec![];

    for (idx, seq) in seqs.iter().rev().enumerate()
    {
        if idx == 0
        {
            predicted_new_col.push(0);
        }
        else 
        {
            let right_val = *seq.first().unwrap();
            let diff = predicted_new_col.last().unwrap();

            predicted_new_col.push(right_val - diff);

        }
    }

    *predicted_new_col.last().unwrap()
}

pub fn predict_next_value(seqs: Vec<Vec<isize>>) -> isize
{
    let mut predicted_new_col = vec![];

    for (idx, seq) in seqs.iter().rev().enumerate()
    {
        if idx == 0
        {
            predicted_new_col.push(0);
        }
        else 
        {
            let left_val = *seq.last().unwrap();
            let diff = predicted_new_col.last().unwrap();

            predicted_new_col.push(left_val + diff);
        }
    }

    *predicted_new_col.last().unwrap()
}

impl History 
{
    pub fn generate_sequences(&self) -> Vec<Vec<isize>>
    {
        let mut sequences = vec![self.values.clone()];

       loop 
        {
            let cur_seq = sequences.last().unwrap();

            if cur_seq.iter().all(|n| *n == 0) {
                break;
            }

            sequences.push((0..cur_seq.len()-1)
                .map(|idx| cur_seq[idx+1] - cur_seq[idx])
                .collect());
        }

        sequences
    }
}

// ----------------------------------------------------
// Model
// ----------------------------------------------------

#[derive(Debug, Clone, Default, PartialEq)]
pub struct History {
    pub values: Vec<isize>
}

// ----------------------------------------------------
// readers
// ----------------------------------------------------

pub fn read_history(in_str: String) -> History
{
    History { values: in_str.split(" ").map(str::parse).map(Result::unwrap).collect() }
}

pub fn read_histories(in_path: &Path) -> Vec<History>
{
    BufReader::new(
        File::open(in_path).expect("Input file does not open")
     )
        .lines()
        .map(Result::unwrap)
        .map(read_history)
        .collect()
}