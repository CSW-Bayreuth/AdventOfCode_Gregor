// ----------------------------------------------------
// ----------------------------------------------------
// imports
// ----------------------------------------------------
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path, cmp::Ordering
};

// ----------------------------------------------------
// App
// ----------------------------------------------------
pub fn start_app() {
   println!(
        "Total winnings of hands: {}",
        total_winnings_of_hands(read_hands(Path::new("./input/aoc_23_07/input.txt")))
    );
/*  
    println!(
        "Number of ways the record could be beaten in this race: {}",
        read_race_info_ignore_kerning(Path::new("./input/aoc_23_06/input.txt")).number_of_ways_to_beat_record()
    ); */
}

// ----------------------------------------------------
// Evaluation
// ----------------------------------------------------

pub fn total_winnings_of_hands(hands: Vec<Hand>) -> usize
{
    let mut hands_working = hands.clone();
    hands_working.sort_by(Hand::cmp_strength);

    hands_working
        .iter()
        .enumerate()
        .map(|(n, hand)| (n+1) * hand.bid)
        .sum()
}

// ----------------------------------------------------
// Model
// ----------------------------------------------------

const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'
];

pub fn card_idx(card: char) -> usize {
    for (index, c) in CARDS.iter().enumerate() {
        if *c == card {
            return index;
        }
    }
    0
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Hand {
    pub cards: [char; 5],
    pub bid: usize
}

impl Hand {
    pub fn cmp_strength(&self, other: &Self) -> Ordering 
    {
        if self.hand_type() != other.hand_type() 
        {
            (&self.hand_type()).cmp(&other.hand_type())
        }
        else 
        {
            for index in 0..self.cards.len()
            {
                if card_idx(self.cards[index]) > card_idx(other.cards[index])
                {
                    return Ordering::Greater;
                }
                if card_idx(self.cards[index]) < card_idx(other.cards[index])
                {
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        }
    }

    pub fn hand_type(&self) -> HandType
    {
        let mut counts_to_cards 
            = vec![vec![],vec![],vec![],vec![],vec![],vec![]];

        // collect card type counts
        for card in CARDS.iter()
        {
            let card_count = self.cards
                .iter()
                .filter(|c| **c == *card)
                .count();

            counts_to_cards[card_count].push(card);
        }

        // evaluate
        if counts_to_cards[5].len() == 1 {
            return HandType::FiveOfAKind
        }
        if counts_to_cards[4].len() == 1 {
            return HandType::FourOfAKind
        }
        if counts_to_cards[3].len() == 1 && counts_to_cards[2].len() == 1 {
            return HandType::FullHouse
        }
        if counts_to_cards[3].len() == 1 {
            return HandType::ThreeOfAKind
        }
        if counts_to_cards[2].len() == 2 {
            return HandType::TwoPair
        }
        if counts_to_cards[2].len() == 1 {
            return HandType::OnePair
        }
        HandType::HighCard
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum HandType {
    HighCard,
    OnePair, 
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind, 
    FiveOfAKind
}

// ----------------------------------------------------
// readers
// ----------------------------------------------------

pub fn read_hand(in_str: String) -> Hand
{
    let split = in_str.split(" ").collect::<Vec<&str>>();

    Hand { 
        cards: split[0].chars().collect::<Vec<char>>().try_into().unwrap(), 
        bid: split[1].parse().unwrap()
    }
}

pub fn read_hands(in_path: &Path) -> Vec<Hand>
{
    BufReader::new(
        File::open(in_path).expect("Input file does not open")
     )
        .lines()
        .map(Result::unwrap)
        .map(read_hand)
        .collect()
}
