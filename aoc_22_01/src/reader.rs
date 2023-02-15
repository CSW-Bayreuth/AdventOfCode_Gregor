use super::model::Elf;
use super::model::FoodRation;
use super::utils::PushReturn;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// ----------------------------------------------------
pub fn read_elfs_and_foodrations(filepath: &Path, elflist: &mut Vec<Elf>) {
    let mut currentelf = elflist.push_return(Elf::default());

    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            if let Ok(line_content) = line {
                match line_content.as_str().trim() {
                    "" => currentelf = elflist.push_return(Elf::default()),
                    calories => read_foodration_for_elf(&mut currentelf, calories),
                }
            }
        }
    }
}

fn read_foodration_for_elf(elf: &mut Elf, calories: &str) {
    elf.inventory.push(FoodRation {
        calories: calories
            .parse()
            .expect(format!("Expected {:?} to be an integer", calories).as_str()),
    });
}

// ----------------------------------------------------
fn read_lines(filepath: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
