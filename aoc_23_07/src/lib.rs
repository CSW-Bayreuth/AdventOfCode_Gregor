// ----------------------------------------------------
// ----------------------------------------------------
// imports
// ----------------------------------------------------
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path, cmp::Ordering
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator, IndexedParallelIterator};

// ----------------------------------------------------
// App
// ----------------------------------------------------
pub fn start_app() {
   println!(
        "Total winnings of hands: {}",
        total_winnings_of_hands(read_hands(Path::new("./input/aoc_23_07/input.txt")), false)
    );
  
    println!(
        "Total winnings of hands with joker mechanic: {}",
        total_winnings_of_hands(read_hands(Path::new("./input/aoc_23_07/input.txt")), true)
    );
}

// ----------------------------------------------------
// Evaluation
// ----------------------------------------------------

pub fn total_winnings_of_hands(hands: Vec<Hand>, joker_active: bool) -> usize
{
    let mut hands_working = hands.clone();

    hands_working.sort_by(
        if joker_active {Hand::cmp_strength_joker} 
        else {Hand::cmp_strength_regular}
    );

    hands_working
        .par_iter()
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
pub fn card_idx_joker(card: char) -> usize 
{
    for (index, c) in CARDS.iter().enumerate() 
    {
        if *c == card {
            if *c == 'J' {
                return 0;
            }
            else {
                if index < card_idx('J') {
                    return index + 1;
                }
                else {
                    return index;
                }
            }
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
    pub fn cmp_strength_regular(&self, other: &Self) -> Ordering 
    {
        self.cmp_strength_generic(other, card_idx, Hand::hand_type_regular)
    }

    pub fn cmp_strength_joker(&self, other: &Self) -> Ordering 
    {
        self.cmp_strength_generic(other, card_idx_joker, Hand::hand_type_joker)
    }

    pub fn cmp_strength_generic(&self, 
        other: &Self, 
        card_idx_func: fn(char) -> usize,
        hand_type_func: fn(&Hand) -> HandType
    ) -> Ordering 
    {
        let hand_type_self = hand_type_func(self);
        let hand_type_other = hand_type_func(other);

        if hand_type_self != hand_type_other
        {
            hand_type_self.cmp(&hand_type_other)
        }
        else 
        {
            for index in 0..self.cards.len()
            {
                if card_idx_func(self.cards[index]) > card_idx_func(other.cards[index])
                {
                    return Ordering::Greater;
                }
                if card_idx_func(self.cards[index]) < card_idx_func(other.cards[index])
                {
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        }
    }


    pub fn hand_type_joker(&self) -> HandType
    {
        self.hand_type_joker_rec(0)
    }
    fn hand_type_joker_rec(&self, index: usize) -> HandType
    {
        let next_hand_type_func = 
            if index == 4 
                {|hand: &Hand, _: usize|{hand.hand_type_regular()}}
            else
                {Hand::hand_type_joker_rec};

        if self.cards[index] == 'J' 
        {
            CARDS
                .par_iter()
                .map(|card|
                {
                    if *card == 'J' {
                        HandType::HighCard
                    }
                    else {
                        let mut modified_hand = self.clone();
                        modified_hand.cards[index] = *card;
                        next_hand_type_func(&modified_hand, index + 1)
                    }
                })
                .max()
                .unwrap()
        }
        else 
        {
            next_hand_type_func(self, index + 1)
        }
    }

    pub fn hand_type_regular(&self) -> HandType
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
