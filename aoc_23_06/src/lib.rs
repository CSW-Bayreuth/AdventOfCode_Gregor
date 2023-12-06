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
pub fn start_app() {
    println!(
        "Product of number of ways the record could be beaten in each race: {}",
        product_of_numbers_of_ways_to_beat_records(read_race_infos(Path::new("./input/aoc_23_06/input.txt")))
    );

    println!(
        "Number of ways the record could be beaten in this race: {}",
        read_race_info_ignore_kerning(Path::new("./input/aoc_23_06/input.txt")).number_of_ways_to_beat_record()
    );
}

// ----------------------------------------------------
// Evaluation
// ----------------------------------------------------

pub fn product_of_numbers_of_ways_to_beat_records(races: Vec<RaceInfo>) -> usize 
{
    races
        .iter()
        .map(RaceInfo::number_of_ways_to_beat_record)
        .fold(1, |acc, n| acc * n)
}

// ----------------------------------------------------
// Model
// ----------------------------------------------------

#[derive(Debug, PartialEq, Clone, Default)]
pub struct RaceInfo {
    pub time: usize,
    pub record_distance: usize
}

impl RaceInfo {
    pub fn number_of_ways_to_beat_record(&self) -> usize 
    {
        (0..self.time)
            .map(|hold_time|{
                let mut boat = ToyBoat::default();
                boat.hold_button(hold_time);
                boat.travel_and_return_dist(self.time - hold_time)
            })
            .filter(|dist| *dist > self.record_distance)
            .count()
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ToyBoat {
    pub speed: usize
}

impl ToyBoat {
    pub fn hold_button(&mut self, time: usize) 
    {
        self.speed += time;
    }

    pub fn travel_and_return_dist(&self, time: usize) -> usize 
    {
        self.speed * time
    }
}

// ----------------------------------------------------
// readers
// ----------------------------------------------------

pub fn read_race_infos(in_path: &Path) -> Vec<RaceInfo>
{
    let lines_with_ints = BufReader::new(
        File::open(in_path).expect("Input file does not open")
     )
        .lines()
        .map(Result::unwrap)
        .map(|line|{
            line
                .split(" ")
                .map(|s| s.parse::<usize>())
                .filter(Result::is_ok)
                .map(Result::unwrap)
                .collect()
        }).collect::<Vec<Vec<usize>>>();
        
    let mut res = vec![];

    for index in 0..lines_with_ints[0].len()
    {
        res.push(RaceInfo { time: lines_with_ints[0][index], record_distance: lines_with_ints[1][index] })
    }

    res
}

pub fn read_race_info_ignore_kerning(in_path: &Path) -> RaceInfo
{
    let lines_with_ints = BufReader::new(
        File::open(in_path).expect("Input file does not open")
     )
        .lines()
        .map(Result::unwrap)
        .map(|line|{
            line
                .split(" ")
                .filter(|s| s.parse::<usize>().is_ok())
                .collect::<String>()
        }).map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();
        
    RaceInfo { time: lines_with_ints[0], record_distance: lines_with_ints[1] }
}
