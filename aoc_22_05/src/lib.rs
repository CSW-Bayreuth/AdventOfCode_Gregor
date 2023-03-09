// ----------------------------------------------------
// imports and re-exports
// ----------------------------------------------------
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;

// ----------------------------------------------------
// types + traits
// ----------------------------------------------------

// ----------------------------------------------------
pub type Cargo = char;
pub type Stack = Vec<Cargo>;
pub type Supplies = Vec<Stack>;
pub type CraneCmd = (usize, usize, usize);

// ----------------------------------------------------
pub trait Reader<T> {
    fn read(&self) -> Option<T>;
}

// ----------------------------------------------------
// starter func
// ----------------------------------------------------
pub fn start_app() {
    println!("Hello World!");
}

// ----------------------------------------------------
// reader utils
// ----------------------------------------------------

// ----------------------------------------------------
// reader trait implementations
// ----------------------------------------------------

impl Reader<Cargo> for String {
    fn read(&self) -> Option<Cargo> {
        None
    }
}

impl Reader<CraneCmd> for String {
    fn read(&self) -> Option<CraneCmd> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d) from (\d to (\d)").unwrap();
        }
        const ERR_MSG: &str = "crane cmd should have format 'move UINT from UINT to UINT'";

        RE.captures(self)?
            .iter()
            .map(|c| c.expect(ERR_MSG).as_str().parse::<usize>().expect(ERR_MSG))
            .next_tuple()
    }
}
