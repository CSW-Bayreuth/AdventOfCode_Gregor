// ----------------------------------------------------
// imports
// ----------------------------------------------------
use crate::model::IdRange;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

// ----------------------------------------------------
// reader trait + implementations
// ----------------------------------------------------
pub trait Reader<T> {
    fn read(&self) -> T;
}

// ----------------------------------------------------
impl Reader<Vec<(IdRange, IdRange)>> for &Path {
    fn read(&self) -> Vec<(IdRange, IdRange)> {
        io::BufReader::new(File::open(self).unwrap())
            .lines()
            .map(Result::unwrap)
            .map(String::from)
            .filter(|s| !s.is_empty())
            .map(|ref l| l.read())
            .collect::<Vec<(IdRange, IdRange)>>()
    }
}

// ----------------------------------------------------
impl Reader<(IdRange, IdRange)> for String {
    fn read(&self) -> (IdRange, IdRange) {
        let two_id_ranges = self
            .split(',')
            .map(|r| String::from(r))
            .collect::<Vec<String>>();

        assert_eq!(
            two_id_ranges.len(),
            2,
            "input does not contain idrange pairs only"
        );

        (two_id_ranges[0].read(), two_id_ranges[1].read())
    }
}

// ----------------------------------------------------
impl Reader<IdRange> for String {
    fn read(&self) -> IdRange {
        let two_ids = self
            .split('-')
            .map(String::from)
            .map(|r| r.parse())
            .map(Result::unwrap)
            .collect::<Vec<usize>>();

        assert_eq!(
            two_ids.len(),
            2,
            "input does not contain idranges with two ids only"
        );

        IdRange(two_ids[0], two_ids[1])
    }
}
