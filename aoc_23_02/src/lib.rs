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
        sum_of_possible_game_ids(read_puzzleinput(Path::new("./input/aoc_23_02/input.txt")), HandfulOfCubes(12, 13, 14))
    );
    println!(
        "Sum of powers of minimum sets of cubes required for each game: {}",
        power_of_min_required_cube_count(read_puzzleinput(Path::new("./input/aoc_23_02/input.txt")))
    );
}

// ----------------------------------------------------
// Evaluation
// ----------------------------------------------------

pub fn sum_of_possible_game_ids(puzzle_input: PuzzleInput, cube_count: HandfulOfCubes) -> usize
{
    puzzle_input.games
        .iter()
        .filter(|game| is_game_possible(&game.handfuls, cube_count))
        .map(|game| game.id)
        .sum()
}

pub fn is_game_possible(handfuls: &Vec<HandfulOfCubes>, cube_count: HandfulOfCubes) -> bool
{
    handfuls
        .iter()
        .all(|handful| {
            handful.0 <= cube_count.0 &&
            handful.1 <= cube_count.1 &&
            handful.2 <= cube_count.2
        })
}

pub fn power_of_min_required_cube_count(puzzle_input: PuzzleInput) -> usize
{
    puzzle_input.games
        .iter()
        .map(|game| min_required_cube_count(&game.handfuls))
        .map(|min_count| min_count.power())
        .sum()
}

pub fn min_required_cube_count(handfuls: &Vec<HandfulOfCubes>) -> HandfulOfCubes
{
    handfuls
        .iter()
        .fold(HandfulOfCubes::default(), |mut acc, handful| {
            acc.0 = usize::max(acc.0, handful.0);
            acc.1 = usize::max(acc.1, handful.1);
            acc.2 = usize::max(acc.2, handful.2);
            acc
        })
}

// ----------------------------------------------------
// Model
// ----------------------------------------------------

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CubeColorCount {
    Red(usize),
    Green(usize),
    Blue(usize)
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct HandfulOfCubes (
    pub usize,
    pub usize,
    pub usize,
);

impl HandfulOfCubes {
    pub fn power(&self) -> usize {
        self.0 * self.1 * self.2
    }
}

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

pub fn read_cubecount(in_str: &str) -> CubeColorCount 
{
    let as_str = in_str.to_string();
    let mut split_res = as_str.trim().split(" ");

    let count: usize = split_res.next().unwrap().parse().unwrap_or(0);
    let color = split_res.next().unwrap();

    match color {
        "red" => CubeColorCount::Red(count),
        "green" => CubeColorCount::Green(count),
        "blue" => CubeColorCount::Blue(count),
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
            CubeColorCount::Red(count) => res.0 += count,
            CubeColorCount::Green(count) => res.1 += count,
            CubeColorCount::Blue(count) => res.2 += count,
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