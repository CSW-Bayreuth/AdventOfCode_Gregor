// ----------------------------------------------------
// ----------------------------------------------------
// imports
// ----------------------------------------------------
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path, vec
};

// ----------------------------------------------------
// App
// ----------------------------------------------------
pub fn start_app() {
    println!(
        "Total worth of cards: {}",
        worth_of_cards(&read_cards(Path::new("./input/aoc_23_04/input.txt")))
    );

    println!(
       "Remaining scratchcards after scratchcard evolution: {}",
       scratchcard_evolution(&read_cards(Path::new("./input/aoc_23_04/input.txt")))
    );
}

// ----------------------------------------------------
// Evaluation
// ----------------------------------------------------

pub fn scratchcard_evolution(in_cards: &Vec<Card>) -> usize
{
    let mut cards = in_cards.clone();

    let mut index = 0;

    loop
    {
        let num_matches = cards[index]
            .matching_numbers()
            .len();

        let id = cards[index].id;

        for clone_index in id .. id+num_matches 
        {
            if clone_index < in_cards.len()
            {
                cards.push(cards[clone_index].clone());
            }
        }

        index += 1;

        if index == cards.len() 
            { break; }
    }

    cards.len()
}

pub fn worth_of_cards(cards: &Vec<Card>) -> usize 
{
    cards
        .iter()
        .map(Card::worthiness)
        .sum()
}

// ----------------------------------------------------
// Model
// ----------------------------------------------------

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Card {
    pub id: usize,
    pub winning_numbers: Vec<usize>,
    pub numbers_you_have: Vec<usize>,
}

impl Card {
    pub fn worthiness(&self) -> usize
    {
        let base: usize = 2;

        let len: u32 = self
            .matching_numbers()
            .len()
            .try_into()
            .unwrap();
        
        if len == 0 
            { 0 } 
        else 
            { base.pow(len - 1) }
    }
    
    pub fn matching_numbers(&self) -> Vec<usize>
    {
        self.winning_numbers
            .iter()
            .fold(vec![], |mut matches, win_num|
            {
                if self.numbers_you_have.contains(win_num) 
                    && !matches.contains(win_num)
                { 
                    matches.push(*win_num);
                }
                matches
            })
    }
}

// ----------------------------------------------------
// readers
// ----------------------------------------------------

pub fn read_cards(in_path: &Path) -> Vec<Card>
{
    BufReader::new(
        File::open(in_path).expect("Input file does not open")
     )
        .lines()
        .map(Result::unwrap)
        .map(read_card)
        .collect()
}

pub fn read_card(in_str: String) -> Card
{
    let mut card = Card::default();
    
    let split_level1 = in_str
        .split(":")
        .map(str::to_string)
        .collect::<Vec<String>>();
    
    card.id = split_level1[0]
        .split(" ")
        .map(str::to_string)
        .collect::<Vec<String>>()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let split_level22 = split_level1[1]
        .split("| ")
        .map(str::to_string)
        .collect::<Vec<String>>();

    card.winning_numbers = read_numbers(split_level22[0].clone());
    card.numbers_you_have = read_numbers(split_level22[1].clone());

    card
}

pub fn read_numbers(in_str: String) -> Vec<usize>
{
    in_str
        .split(" ")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}
