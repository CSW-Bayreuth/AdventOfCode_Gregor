use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::model::{Compartment, Knapsack};

use super::model::Item;

pub trait Reader<T> {
    fn read(&self) -> T;
}

impl Reader<Vec<Knapsack>> for &Path {
    fn read(&self) -> Vec<Knapsack> {
        io::BufReader::new(File::open(self).unwrap())
            .lines()
            .map(Result::unwrap)
            .map(String::from)
            .filter(|s| !s.is_empty())
            .map(|ref l| l.read())
            .collect::<Vec<Knapsack>>()
    }
}

impl Reader<Knapsack> for String {
    fn read(&self) -> Knapsack {
        assert_eq!(
            self.len() % 2,
            0,
            "knapsack parse data len must be a multiple of 2"
        );

        let data_c1 = &self[..self.len() / 2];
        let data_c2 = &self[self.len() / 2..];

        Knapsack(data_c1.read(), data_c2.read())
    }
}

impl Reader<Compartment> for &str {
    fn read(&self) -> Compartment {
        Compartment(self.chars().map(|ref c| c.read()).collect::<Vec<Item>>())
    }
}

impl Reader<Item> for char {
    fn read(&self) -> Item {
        assert!(matches!(self.to_ascii_lowercase(), 'a'..='z'));
        Item(*self)
    }
}
