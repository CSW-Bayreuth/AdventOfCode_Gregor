pub struct FoodRation {
    pub calories: u32,
}

impl Default for FoodRation {
    fn default() -> FoodRation {
        FoodRation { calories: 0 }
    }
}

pub struct Elf {
    pub inventory: Vec<FoodRation>,
}

impl Default for Elf {
    fn default() -> Elf {
        Elf {
            inventory: Vec::new(),
        }
    }
}
