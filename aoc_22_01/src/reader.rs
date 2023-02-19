use super::model::Elf;
use super::model::FoodRation;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// ----------------------------------------------------
pub fn read(filepath: &Path) -> Vec<Elf> {
    read_lines(filepath)
        .unwrap()
        .map(Result::unwrap)
        .map(|s| String::from(s.as_str().trim()))
        .fold(vec![Elf::default()], |mut acc_v, s| {
            match s.as_str() {
                "" => acc_v.push(Elf::default()),
                ref calories => read_foodration(acc_v.last_mut().unwrap(), calories),
            }
            acc_v
        })
}

// ----------------------------------------------------
fn read_foodration(elf: &mut Elf, calories: &str) {
    elf.inventory.push(FoodRation {
        calories: calories
            .parse()
            .expect(format!("Expected {:?} to be an integer", calories).as_str()),
    });
}

// ----------------------------------------------------
fn read_lines(filepath: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    Ok(io::BufReader::new(File::open(filepath)?).lines())
}
