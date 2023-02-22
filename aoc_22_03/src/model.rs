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
