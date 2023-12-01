// ----------------------------------------------------
// Imports
// ----------------------------------------------------

// ----------------------------------------------------
// Filesystem
// ----------------------------------------------------
#[derive(PartialEq, PartialOrd, Debug)]
pub enum DirContent<'a> {
    File {
        name: &'a str,
        size: usize,
    },
    Dir {
        name: &'a str,
        content: Vec<&'a mut DirContent<'a>>,
    },
}

// ----------------------------------------------------
// Filesystem: Size Calculations
// ----------------------------------------------------
pub fn size_recursive(content: &DirContent) -> usize {
    match content {
        DirContent::File { size, .. } => *size,
        DirContent::Dir { content, .. } => content
            .iter()
            .fold(0, |acc, item| acc + size_recursive(item)),
    }
}

pub fn sum_sizes_of_dirs_with_size_lower_max(_content: &DirContent, _max_size: usize) -> usize {
    panic!("not implemented");
}
