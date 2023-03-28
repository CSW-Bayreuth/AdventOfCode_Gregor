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
pub fn parse_terminal_line<'a>(line: &'a String) -> TerminaLine {
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

pub fn parse_terminal_output_to_dirtree(input_path_str: &str) -> DirContent {
    panic!("not implemented");
}
