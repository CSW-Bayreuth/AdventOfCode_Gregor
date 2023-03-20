// ----------------------------------------------------
// imports and re-exports
// ----------------------------------------------------
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

// ----------------------------------------------------
// types + traits
// ----------------------------------------------------

// ----------------------------------------------------
pub type Cargo = char;
pub type CargoRow = Vec<Option<Cargo>>;
pub type CargoStack = Vec<Cargo>;
pub type Supplies = Vec<CargoStack>;
pub type CraneCmd = (usize, usize, usize);

// ----------------------------------------------------
pub trait Reader<T> {
    fn read(&self) -> T;
}

// ----------------------------------------------------
// starter func
// ----------------------------------------------------
pub fn start_app() {
    let (mut supplies, crane_cmds): (Supplies, Vec<CraneCmd>) =
        read_supply_situation(Path::new("./input/aoc_22_05/input.txt"));

    println!(
        "Top cargos after reorder according to crane 9000 commands: {}",
        top_cargos_after_cmd_apply(
            &mut supplies.clone(),
            &crane_cmds,
            &CraneType::CrateMover9000
        )
    );

    println!(
        "Top cargos after reorder according to crane 9001 commands: {}",
        top_cargos_after_cmd_apply(&mut supplies, &crane_cmds, &CraneType::CrateMover9001)
    );
}

// ----------------------------------------------------
// utils
// ----------------------------------------------------

pub enum CraneType {
    CrateMover9000,
    CrateMover9001,
}

pub fn top_cargos_after_cmd_apply(
    supplies: &mut Supplies,
    crane_cmds: &Vec<CraneCmd>,
    crane_type: &CraneType,
) -> String {
    apply_crane_cmds(supplies, crane_cmds, &crane_type);

    supplies
        .iter()
        .map(|v| v.iter().last())
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect()
}

pub fn apply_crane_cmds(
    supplies: &mut Supplies,
    crane_cmds: &Vec<CraneCmd>,
    crane_type: &CraneType,
) {
    for cmd in crane_cmds {
        apply_crane_cmd(supplies, cmd, crane_type);
    }
}

pub fn apply_crane_cmd(supplies: &mut Supplies, crane_cmd: &CraneCmd, crane_type: &CraneType) {
    let (n_crates, stack1, stack2) = crane_cmd;

    let idx = supplies[stack1 - 1].len() - n_crates;

    for _ in 0..*n_crates {
        let cargo = match crane_type {
            CraneType::CrateMover9000 => supplies[stack1 - 1].pop().unwrap(),
            CraneType::CrateMover9001 => supplies[stack1 - 1].remove(idx),
        };
        supplies[stack2 - 1].push(cargo);
    }
}

// ----------------------------------------------------
// partial consumer trait implementations
// ----------------------------------------------------

pub fn read_supply_situation(path: &Path) -> (Supplies, Vec<CraneCmd>) {
    consume_read_supply_situation(
        BufReader::new(File::open(path).unwrap())
            .lines()
            .map(Result::unwrap),
    )
}

pub fn consume_read_supply_situation(
    mut iter: impl Iterator<Item = String>,
) -> (Supplies, Vec<CraneCmd>) {
    let supplies = consume_read_supplies(&mut iter);
    consume_delimiting_row(&mut iter);
    let crane_cmds = consume_read_crane_cmds(&mut iter);
    (supplies, crane_cmds)
}

pub fn consume_read_supplies(iter: &mut impl Iterator<Item = String>) -> Supplies {
    let mut supplies: Supplies = Vec::new();

    while let Some(cargo_row) = iter.next().unwrap().read() {
        // add new cargo stacks to the right if needed
        if cargo_row.len() > supplies.len() {
            supplies.resize_with(cargo_row.len(), || Vec::new());
        }

        // add each cargo in the row to the respective stack
        for (i, cargo_opt) in cargo_row.iter().enumerate() {
            if let Some(cargo) = cargo_opt {
                supplies[i].insert(0, *cargo);
            }
        }
    }

    supplies
}

pub fn consume_delimiting_row(iter: &mut impl Iterator<Item = String>) -> () {
    let _ = iter.next();
}

pub fn consume_read_crane_cmds(iter: &mut impl Iterator<Item = String>) -> Vec<CraneCmd> {
    iter.map(|s| s.read()).collect()
}

// ----------------------------------------------------
// reader trait implementations
// ----------------------------------------------------

impl Reader<Option<CargoRow>> for String {
    fn read(&self) -> Option<CargoRow> {
        if self.starts_with(" 1") {
            return None;
        }

        let mut cargo_row: CargoRow = Vec::new();
        let mut cargo_str = String::new();

        let mut chars_iter = self.chars();

        while let Some(ch) = chars_iter.next() {
            cargo_str.push(ch);

            if cargo_str.len() == 3 {
                cargo_row.push(if cargo_str.as_str() == "   " {
                    None
                } else {
                    Some(cargo_str.read())
                });
                cargo_str.clear();
                let _ = chars_iter.next(); // skip delimiter
            }
        }

        Some(cargo_row)
    }
}

impl Reader<Cargo> for String {
    fn read(&self) -> Cargo {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[([[:alpha:]])\]").unwrap();
        }
        const ERR_MSG_ALPHA: &str = "cargo should have format '[a-zA-Z]'";

        RE.captures_iter(self)
            .next()
            .expect(ERR_MSG_ALPHA)
            .iter() // till here: get first capture
            .skip(1) // skip full string
            .next()
            .expect(ERR_MSG_ALPHA)
            .expect(ERR_MSG_ALPHA) // till here: get first match
            .as_str()
            .chars()
            .next()
            .expect(ERR_MSG_ALPHA)
    }
}

impl Reader<CraneCmd> for String {
    fn read(&self) -> CraneCmd {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move ([\d]+) from ([\d]+) to ([\d]+)").unwrap();
        }
        const ERR_MSG_UINT: &str = "crane cmd should have format 'move UINT from UINT to UINT'";

        RE.captures_iter(self)
            .map(|m| {
                m.iter()
                    .skip(1)
                    .map(|c| {
                        c.expect(ERR_MSG_UINT)
                            .as_str()
                            .parse::<usize>()
                            .expect(ERR_MSG_UINT)
                    })
                    .next_tuple::<CraneCmd>()
            })
            .next()
            .unwrap()
            .unwrap()
    }
}
