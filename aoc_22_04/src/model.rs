// ----------------------------------------------------
// ID range
// ----------------------------------------------------
#[derive(Debug)]
pub struct IdRange(pub usize, pub usize);

impl IdRange {
    pub fn is_in(&self, other: &IdRange) -> bool {
        other.0 <= self.0 && self.1 <= other.1
    }
}

impl PartialEq for IdRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
