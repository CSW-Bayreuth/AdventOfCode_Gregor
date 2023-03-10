// ----------------------------------------------------
// imports and re-exports
// ----------------------------------------------------
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
// use std::path::Path;

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
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[([[:alpha:]])\]").unwrap();
        }
        const ERR_MSG_MATCH: &str = "cargo input match did not unwrap";
        const ERR_MSG_ALPHA: &str = "cargo should have format '[a-zA-Z]'";

        RE.captures_iter(self)
            .next()
            .expect(ERR_MSG_MATCH)
            .iter()
            .next()
            .expect(ERR_MSG_MATCH)
            .expect(ERR_MSG_ALPHA)
            .as_str()
            .chars()
            .next()
    }
}

impl Reader<CraneCmd> for String {
    fn read(&self) -> Option<CraneCmd> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d) from (\d) to (\d)").unwrap();
        }
        const ERR_MSG_MATCH: &str = "crane cmd input match did not unwrap";
        const ERR_MSG_UINT: &str = "crane cmd should have format 'move UINT from UINT to UINT'";

        RE.captures_iter(self)
            .map(|m| {
                m.iter()
                    .skip(1)
                    .map(|c| {
                        c.expect(ERR_MSG_MATCH)
                            .as_str()
                            .parse::<usize>()
                            .expect(ERR_MSG_UINT)
                    })
                    .next_tuple::<CraneCmd>()
            })
            .next()
            .unwrap()
    }
}
