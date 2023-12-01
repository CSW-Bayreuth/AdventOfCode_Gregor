/* #![allow(dead_code)]
#![allow(non_snake_case)]
#![cfg(test)]
// ----------------------------------------------------
// imports
// ----------------------------------------------------
use rstest::rstest;

use aoc_22_07::*;

// ----------------------------------------------------
// tests constants
// ----------------------------------------------------
const DIRECTORY_E: DirContent = DirContent::Dir {
    name: "e",
    content: vec![&DirContent::File {
        name: "i",
        size: 584,
    }],
};

const DIRECTORY_A: DirContent = DirContent::Dir {
    name: "a",
    content: vec![
        &DIRECTORY_E,
        &DirContent::File {
            name: "f",
            size: 29116,
        },
        &DirContent::File {
            name: "g",
            size: 2557,
        },
        &DirContent::File {
            name: "h.lst",
            size: 62596,
        },
    ],
};

const DIRECTORY_D: DirContent = DirContent::Dir {
    name: "d",
    content: vec![
        &DirContent::File {
            name: "j",
            size: 4060174,
        },
        &DirContent::File {
            name: "d.log",
            size: 8033020,
        },
        &DirContent::File {
            name: "d.ext",
            size: 5626152,
        },
        &DirContent::File {
            name: "k",
            size: 7214296,
        },
    ],
};

const DIRECTORY_ROOT: DirContent = DirContent::Dir {
    name: "/",
    content: vec![
        &DIRECTORY_A,
        &DirContent::File {
            name: "b.txt",
            size: 14848514,
        },
        &DirContent::File {
            name: "c.dat",
            size: 8504156,
        },
        &DIRECTORY_D,
    ],
};

// ----------------------------------------------------
// tests funcs
// ----------------------------------------------------
#[rstest]
#[case("$ cd x", TerminaLine::Command(TerminalCommand::CD(CDTarget::In("x"))))]
#[case("$ cd ..", TerminaLine::Command(TerminalCommand::CD(CDTarget::Out)))]
#[case(
    "$ cd /",
    TerminaLine::Command(TerminalCommand::CD(CDTarget::Outermost))
)]
#[case("$ ls", TerminaLine::Command(TerminalCommand::LS))]
#[case("123 abc", TerminaLine::LSContent(DirContent::File{name: "abc", size: 123}))]
#[case("dir xyz", TerminaLine::LSContent(DirContent::Dir{name: "xyz", content: vec![]}))]
fn test_parse_terminal_line(#[case] input: &str, #[case] expected: TerminaLine) {
    assert_eq!(parse_terminal_line(input), expected);
}

#[rstest]
#[case("./../input/aoc_22_07/input_example.txt", DIRECTORY_ROOT)]
fn test_parse_terminal_output_to_dirtree(
    #[case] input_path_str: &str,
    #[case] expected: DirContent,
) {
    assert_eq!(
        parse_terminal_output_to_dirtree(&read_terminal_lines_from_file(input_path_str)),
        expected
    );
}

#[rstest]
#[case(DIRECTORY_E, 584)]
#[case(DIRECTORY_A, 94853)]
#[case(DIRECTORY_D, 24933642)]
#[case(DIRECTORY_ROOT, 48381165)]
fn test_directory_size(#[case] input_directory: DirContent, #[case] expected: usize) {
    assert_eq!(size_recursive(&input_directory), expected);
}

#[rstest]
#[case(DIRECTORY_ROOT, 100000, 95437)]
fn test_sum_sizes_of_dirs_with_size_lower_max(
    #[case] input_directory: DirContent,
    #[case] input_max: usize,
    #[case] expected: usize,
) {
    assert_eq!(
        sum_sizes_of_dirs_with_size_lower_max(&input_directory, input_max),
        expected,
    );
}
 */