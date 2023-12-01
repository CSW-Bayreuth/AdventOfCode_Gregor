/* use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Deref, DerefMut},
}; */

// ----------------------------------------------------
// Imports
// ----------------------------------------------------
use super::DirContent;

// ----------------------------------------------------
// Terminal Output
// ----------------------------------------------------
#[derive(PartialEq, PartialOrd, Debug)]
pub enum TerminaLine<'a> {
    Command(TerminalCommand<'a>),
    LSContent(DirContent<'a>),
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum TerminalCommand<'a> {
    CD(CDTarget<'a>),
    LS,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum CDTarget<'a> {
    In(&'a str),
    Out,
    Outermost,
}

// ----------------------------------------------------
// Terminal Output: Parsing
// ----------------------------------------------------
pub fn parse_terminal_line(line: &str) -> TerminaLine {
    let mut sections = line.split(" ");

    match sections.next().unwrap() {
        "$" => TerminaLine::Command(match sections.next().unwrap() {
            "cd" => TerminalCommand::CD(match sections.next().unwrap() {
                "/" => CDTarget::Outermost,
                ".." => CDTarget::Out,
                dir => CDTarget::In(dir),
            }),
            "ls" => TerminalCommand::LS,
            _ => panic!("terminal command unknown"),
        }),
        ls_content_first => TerminaLine::LSContent(match ls_content_first {
            "dir" => DirContent::Dir {
                name: sections.next().unwrap(),
                content: vec![],
            },
            size => DirContent::File {
                name: sections.next().unwrap(),
                size: size.parse::<usize>().unwrap(),
            },
        }),
    }
}

/* pub fn parse_terminal_output_to_dirtree<'a>(
    terminal_lines: &'a mut Vec<TerminaLine<'a>>,
) -> DirContent<'a> {
    let mut dirtree: DirContent = DirContent::Dir {
        name: "/",
        content: vec![],
    };
    let mut cur_dir = RefCell::new(&mut dirtree);

    for line in terminal_lines.iter_mut() {
        match line {
            TerminaLine::Command(TerminalCommand::CD(CDTarget::Outermost)) => (),
            TerminaLine::Command(TerminalCommand::CD(CDTarget::Out)) => (),
            TerminaLine::Command(TerminalCommand::CD(CDTarget::In(dir_name))) => {
                let mut cd_dir: Option<RefCell<&mut DirContent>> = None;

                if let DirContent::Dir { ref content, .. } = cur_dir.borrow_mut().deref_mut() {
                    for mut c in content {
                        if let DirContent::Dir { ref name, .. } = c {
                            if name == dir_name {
                                cd_dir = Some(RefCell::new(*c));
                            }
                        }
                    }
                }
                cur_dir = cd_dir.unwrap();
            }
            TerminaLine::Command(TerminalCommand::LS) => (),
            TerminaLine::LSContent(dir_content) => {
                let mut a = cur_dir.borrow_mut().deref_mut();
                if let DirContent::Dir {
                    ref mut content, ..
                } = a
                {
                    content.push(dir_content);
                }
            }
        }
    }

    dirtree
}

pub fn read_terminal_lines_from_file(input_path_str: &str) -> Vec<TerminaLine> {
    BufReader::new(File::open(input_path_str).expect("file could not be found"))
        .lines()
        .skip(1)
        .map(Result::unwrap)
        .map(|s| s.as_str())
        .map(parse_terminal_line)
        .collect::<Vec<TerminaLine>>()
}
 */