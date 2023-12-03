// ----------------------------------------------------
// ----------------------------------------------------
// imports
// ----------------------------------------------------
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path
};

// ----------------------------------------------------
// App
// ----------------------------------------------------
pub fn start_app() {
    println!(
        "Sum of all of the part numbers in the engine schematic: {}",
        sum_of_part_numbers(Path::new("./input/aoc_23_03/input.txt"))
    );
    println!(
        "Sum of all of the gear ratios in the engine schematic: {}",
        sum_of_gear_ratios(Path::new("./input/aoc_23_03/input.txt"))
    );
}

// ----------------------------------------------------
// Evaluation
// ----------------------------------------------------

pub fn sum_of_part_numbers(in_path: &Path) -> usize
{
    let mut schema_grid = read_schemagrid(in_path);
    let schema_nums = extract_schema_numbers(&schema_grid);

    schema_nums
        .iter()
        .filter(|num| num.is_next_to_symbol(&mut schema_grid))
        .map(|num| num.number)
        .sum()
}

pub fn sum_of_gear_ratios(in_path: &Path) -> usize
{
    let mut schema_grid = read_schemagrid(in_path);
    let schema_nums = extract_schema_numbers(&schema_grid); // this sets the adjacent SchemeNumbers of potential gears

    for num in schema_nums {
        num.is_next_to_symbol(&mut schema_grid); // adds SchemaNumbers to adjacent PotentialGears
    }

    schema_grid.cells
        .iter()
        .flatten()
        .filter_map(|cell| {
            if let SchemaCell::SymbolPotentialGear(nums) = cell {
                Some(nums)
            }
            else { None }
        })
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums
            .iter()
            .map(|n| n.number)
            .fold(1, std::ops::Mul::mul)
        )
        .sum()
}

pub fn extract_schema_numbers(schema_grid: &SchemaGrid) -> Vec<SchemaNumber> 
{
    let mut result_numbers = vec![];

    // go through all cells, line by line
    for (y, row) in schema_grid.cells.iter().enumerate()
    {
        let mut is_reading_number = false;
        let mut num = SchemaNumber::default();

        for (x, cell) in row.iter().enumerate()
        {
            // decide what to do next based on cell type
            match cell {
                SchemaCell::Empty | SchemaCell::SymbolPotentialGear(_) | SchemaCell::Symbol => 
                {
                    if is_reading_number {
                        result_numbers.push(num);
                        is_reading_number = false;
                    }
                },

                SchemaCell::Number(n) => 
                {
                    if is_reading_number {
                        num.number = num.number * 10 + n; 
                        num.x_end += 1;
                    }
                    else {
                        num = SchemaNumber { 
                            number: *n, 
                            x_start: x,
                            x_end: x, 
                            y: y 
                        };
                        is_reading_number = true;
                    }
                },
            }
        }

        // at end of row, there might be a pending number
        if is_reading_number {
            result_numbers.push(num);
        }
    }

    result_numbers
}

// ----------------------------------------------------
// Model
// ----------------------------------------------------

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct SchemaNumber {
    pub number: usize,
    pub x_start: usize,
    pub x_end: usize,
    pub y: usize,
}

impl SchemaNumber {

    fn is_cell_symbol(&self, schema_grid: &mut SchemaGrid, x: usize, y: usize) -> bool 
    {
        match schema_grid.cells[y][x] {
            SchemaCell::Symbol => true,
            SchemaCell::SymbolPotentialGear(ref mut v) => { v.push(*self); true },
            _ => false
        }
    }

    pub fn is_next_to_symbol(&self, schema_grid: &mut SchemaGrid) -> bool
    {
        let x_start = if self.x_start > 0                  { self.x_start - 1 } else { self.x_start };
        let x_end   = if self.x_end < schema_grid.size_x-1 { self.x_end   + 1 } else { self.x_end   };

        for x in x_start ..= x_end
        {
            if self.y > 0 {
                if self.is_cell_symbol(schema_grid, x, self.y - 1) 
                    { return true; }
            }

            if self.y < schema_grid.size_y-1 {
                if self.is_cell_symbol(schema_grid, x, self.y + 1) 
                    { return true; }
            }
        }

        // left and right cells of number
        
        if self.x_start > 0 {
            if self.is_cell_symbol(schema_grid, self.x_start - 1, self.y) 
                { return true; }
        }
        
        if self.x_end < schema_grid.size_x-1 {
            if self.is_cell_symbol(schema_grid, self.x_end + 1, self.y) 
                { return true; }
        }

        false
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SchemaCell {
    Number(usize),
    Empty,
    SymbolPotentialGear(Vec<SchemaNumber>),
    Symbol
}

impl Default for SchemaCell {
    fn default() -> Self { SchemaCell::Empty }
}

#[derive(Debug, Default, PartialEq)]
pub struct SchemaGrid {
    pub size_x: usize,
    pub size_y: usize,
    pub cells: Vec<Vec<SchemaCell>>
}

// ----------------------------------------------------
// readers
// ----------------------------------------------------

pub fn read_schemagrid(in_path: &Path) -> Box<SchemaGrid>
{
    let mut schema_grid = Box::new(SchemaGrid::default());

    let lines = BufReader::new(
        File::open(in_path).expect("Input file does not open")
    ).lines();

    for line in lines
    {
        schema_grid.cells.push(vec![]);
        let cell_row = schema_grid.cells.last_mut().unwrap();

        for c in line.expect("Input line can#t be read").chars()
        {
            cell_row.push(read_schemacell(c));
        }
    }

    schema_grid.size_x = schema_grid.cells[0].len();
    schema_grid.size_y = schema_grid.cells.len();

    schema_grid
}

pub fn read_schemacell(in_char: char) -> SchemaCell
{
    match in_char {
        '.' => SchemaCell::Empty,
        '*' => SchemaCell::SymbolPotentialGear(vec![]),
        c if c.is_digit(10) => SchemaCell::Number(c.to_digit(10).unwrap() as usize),
        _ => SchemaCell::Symbol
    }
}