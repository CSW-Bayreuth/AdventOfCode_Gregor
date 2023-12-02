use core::panic;
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
        "Sum of IDs of games which would've been possible if bag were loaded with 12 reds, 13 greens, 14 blues: {}",
        sum_of_possible_game_ids(read_puzzleinput(Path::new("./input/aoc_23_02/input.txt")), 12, 13, 14)
    );
}

// ----------------------------------------------------
// Evaluation
// ----------------------------------------------------

pub fn sum_of_possible_game_ids(puzzle_input: PuzzleInput, num_red: usize, num_green: usize, num_blue: usize) -> usize
{
    puzzle_input.games
        .iter()
        .filter(|game| is_game_possible(&game.handfuls, num_red, num_green, num_blue))
        .map(|game| game.id)
        .sum()
}

pub fn is_game_possible(handfuls: &Vec<HandfulOfCubes>, num_red: usize, num_green: usize, num_blue: usize) -> bool
{
    handfuls
        .iter()
        .all(|handful| {
            handful.0 <= num_red &&
            handful.1 <= num_green &&
            handful.2 <= num_blue
        })
}

// ----------------------------------------------------
// Model
// ----------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum CubeCount {
    Red(usize),
    Green(usize),
    Blue(usize)
}

#[derive(Debug, Default, PartialEq)]
pub struct HandfulOfCubes (
    pub usize,
    pub usize,
    pub usize,
);

#[derive(Debug, Default, PartialEq)]
pub struct Game {
    pub handfuls: Vec<HandfulOfCubes>,
    pub id: usize,
}

#[derive(Debug, Default, PartialEq)]
pub struct PuzzleInput {
    pub games: Vec<Game>
}

// ----------------------------------------------------
// readers
// ----------------------------------------------------

pub fn read_cubecount(in_str: &str) -> CubeCount 
{
    let as_str = in_str.to_string();
    let mut split_res = as_str.trim().split(" ");

    let count: usize = split_res.next().unwrap().parse().unwrap_or(0);
    let color = split_res.next().unwrap();

    match color {
        "red" => CubeCount::Red(count),
        "green" => CubeCount::Green(count),
        "blue" => CubeCount::Blue(count),
        _ => panic!("didn't expect extra color!")
    }
}

// ----------------------------------------------------
pub fn read_handfulofcubes(in_str: &str) -> HandfulOfCubes 
{
    let as_str = in_str.to_string();

    let mut segment_it = as_str.split(", ");
    let mut res = HandfulOfCubes::default();

    while let Some(next_segment) = segment_it.next() 
    {
        match read_cubecount(next_segment) {
            CubeCount::Red(count) => res.0 += count,
            CubeCount::Green(count) => res.1 += count,
            CubeCount::Blue(count) => res.2 += count,
        }
    }

    res
}

// ----------------------------------------------------
pub fn read_game(in_str: String) -> Game 
{
    let id: usize = in_str
        .split(": ")
        .next()
        .unwrap() // get left side
        .chars()
        .skip(5)
        .collect::<String>() // skip 'Game '
        .parse().unwrap();

    let handfuls = in_str
        .split(": ")
        .skip(1)
        .next()
        .unwrap() // get right side
        .split("; ")
        .map(read_handfulofcubes)
        .collect::<Vec<HandfulOfCubes>>()
    ;

    Game { handfuls: handfuls, id: id }
}

// ----------------------------------------------------
pub fn read_puzzleinput(in_path: &Path) -> PuzzleInput 
{
    let games = BufReader::new(File::open(in_path).expect("Input file does not open"))
        .lines()
        .map(|result| result.unwrap())
        .map(read_game)
        .collect::<Vec<Game>>();

    PuzzleInput { games: games }
}