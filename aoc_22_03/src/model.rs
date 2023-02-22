// ----------------------------------------------------
// Elf Group (Iter)
// ----------------------------------------------------
pub struct ElfGroup<'a>(pub &'a Knapsack, pub &'a Knapsack, pub &'a Knapsack);

impl ElfGroup<'_> {
    pub fn iter(&self) -> ElfGroupIter {
        ElfGroupIter {
            current: 0,
            group: vec![self.0, self.1, self.2],
        }
    }
}

pub struct ElfGroupIter<'a> {
    current: usize,
    group: Vec<&'a Knapsack>,
}

impl<'a> Iterator for ElfGroupIter<'a> {
    type Item = &'a Knapsack;
    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        if self.current < 3 {
            result = Some(self.group[self.current]);
            self.current += 1;
        }
        result
    }
}

// ----------------------------------------------------
// Knapsack
// ----------------------------------------------------
pub struct Knapsack {
    pub cp1: Compartment,
    pub cp2: Compartment,
}

impl Knapsack {
    pub fn items(&self) -> Vec<&Item> {
        self.cp1.iter().chain(self.cp2.iter()).collect()
    }
}

// ----------------------------------------------------
// Compartment
// ----------------------------------------------------
pub type Compartment = Vec<Item>;

// ----------------------------------------------------
// Item
// ----------------------------------------------------
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Item(pub char);

// ----------------------------------------------------
impl Item {
    pub fn priority(&self) -> usize {
        ('a'..='z')
            .chain('A'..='Z')
            .enumerate()
            .find(|c| c.1 == self.0)
            .unwrap()
            .0
            + 1
    }
}
