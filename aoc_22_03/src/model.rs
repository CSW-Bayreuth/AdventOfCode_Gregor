pub struct Knapsack(pub Compartment, pub Compartment);

pub struct Compartment(pub Vec<Item>);

pub struct Item(pub char);
