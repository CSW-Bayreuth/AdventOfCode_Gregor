// ----------------------------------------------------
// ----------------------------------------------------
// imports
// ----------------------------------------------------
use std::{
    fs::File, io::{BufRead, BufReader}, path::Path
};

// ----------------------------------------------------
// App
// ----------------------------------------------------
pub fn start_app() 
{
/*     println!(
        "Sum of right extrapolated values: {}",
        sum_of_predicted_values(read_histories(Path::new("./input/aoc_23_09/input.txt")), predict_next_value)
    ); */
    //animal_pos = (25,77)
/*     println!(
        "Sum of left extrapolated values: {}",
        sum_of_predicted_values(read_histories(Path::new("./input/aoc_23_09/input.txt")), predict_previous_value)
    ); */
}

// ----------------------------------------------------
// Evaluation
// ----------------------------------------------------

impl Field {
    pub fn at(&self, pos: Position) -> &Tile
    {
        &self.tiles[pos.1 as usize][pos.0 as usize]
    }

    pub fn animal_tile(&self) -> &Tile {
        &self.at(self.animal_pos)
    }

    pub fn connecting_tiles(&self, tile : &Tile) -> Vec<&Tile> {
        let mut connected_tiles = vec![];

        println!("{:?}", self.at((tile.pos.0 + 1, tile.pos.1)));
        println!("{:?}", self.at((tile.pos.0 - 1, tile.pos.1)));
        println!("{:?}", self.at((tile.pos.0, tile.pos.1 + 1)));
        println!("{:?}", self.at((tile.pos.0, tile.pos.1 - 1)));

        if self.at((tile.pos.0 + 1, tile.pos.1)).connects_west() {
            connected_tiles.push(self.at((tile.pos.0 + 1, tile.pos.1)));
        }
        if self.at((tile.pos.0 - 1, tile.pos.1)).connects_east() {
            connected_tiles.push(self.at((tile.pos.0 - 1, tile.pos.1)));
        }
        if self.at((tile.pos.0, tile.pos.1 + 1)).connects_north() {
            connected_tiles.push(self.at((tile.pos.0, tile.pos.1 + 1)));
        }
        if self.at((tile.pos.0, tile.pos.1 - 1)).connects_south() {
            connected_tiles.push(self.at((tile.pos.0, tile.pos.1 - 1)));
        }

        connected_tiles
    }

    pub fn steps_to_farthest_tile(&self) -> usize
    {
        let mut steps = 0;

        let mut current_tiles = self.connecting_tiles(self.animal_tile());
        let mut visited_tiles = vec![ self.animal_tile() ];

        let mut next_current_tiles = vec![];

        loop 
        {
            steps += 1;

            println!("Current/Visited tiles: {:?} / {:?}", current_tiles, visited_tiles);
            println!("Steps: {:?}", steps);

            for tile in &current_tiles
            {
                println!("Enterable directions: {:?}", tile.enterable_directions());

                for direction in tile.enterable_directions() {
                    if let Some((_, next_pos)) = tile.pipe_forward(direction) {
                        let next_tile = self.at(next_pos);

                        if visited_tiles.contains(&next_tile) {
                            continue;
                        }
                        else {
                            next_current_tiles.push(next_tile);
                        }
                    }
                }
                visited_tiles.push(tile);
            }

            current_tiles = next_current_tiles;
            next_current_tiles = vec![];

            if current_tiles.is_empty() {
                break;
            }
        }

        steps
    }
}

impl Tile {
    pub fn connects_north(&self) -> bool {
        [ TileType::NtoW, TileType::Vert, TileType::NtoE ].contains(&self.tile_type)
    }
    pub fn connects_south(&self) -> bool {
        [ TileType::StoW, TileType::Vert, TileType::StoE ].contains(&self.tile_type)
    }
    pub fn connects_west(&self) -> bool {
        [ TileType::NtoW, TileType::Horiz, TileType::StoW ].contains(&self.tile_type)
    }
    pub fn connects_east(&self) -> bool {
        [ TileType::NtoE, TileType::Horiz, TileType::StoE ].contains(&self.tile_type)
    }

    pub fn enterable_directions(&self) -> Vec<Direction> {
        let mut directions = Vec::new();
        if self.connects_north() {
            directions.push(DIRECTION_NORTH);
        }
        if self.connects_south() {
            directions.push(DIRECTION_SOUTH);
        }
        if self.connects_west() {
            directions.push(DIRECTION_WEST);
        }
        if self.connects_east() {
            directions.push(DIRECTION_EAST);
        }
        directions
    }

    pub fn pipe_forward(&self, abs_entry_direction: Direction) -> Option<(Direction, Position)>
    {
        match abs_entry_direction {
            DIRECTION_EAST => // from west
            {
                match self.tile_type {
                    TileType::NtoW => {
                        return Option::Some((DIRECTION_NORTH, (self.pos.0, self.pos.1 + 1)));
                    },
                    TileType::Horiz => {
                        return Option::Some((DIRECTION_EAST, (self.pos.0 + 1, self.pos.1)));
                    },
                    TileType::StoW => {
                        return Option::Some((DIRECTION_SOUTH, (self.pos.0, self.pos.1 - 1)));
                    },
                    _ => Option::None
                }
            },
            DIRECTION_WEST => // from east,
            {
                match self.tile_type {
                    TileType::NtoE => {
                        return Option::Some((DIRECTION_NORTH, (self.pos.0, self.pos.1 + 1)));
                    },
                    TileType::Horiz => {
                        return Option::Some((DIRECTION_WEST, (self.pos.0 - 1, self.pos.1)));
                    },
                    TileType::StoE => {
                        return Option::Some((DIRECTION_SOUTH, (self.pos.0, self.pos.1 - 1)));
                    },
                    _ => Option::None
                }
            },
            DIRECTION_NORTH => // from south,
            {
                match self.tile_type {
                    TileType::StoE => {
                        return Option::Some((DIRECTION_EAST, (self.pos.0 + 1, self.pos.1)));
                    },
                    TileType::Vert => {
                        return Option::Some((DIRECTION_NORTH, (self.pos.0, self.pos.1 + 1)));
                    },
                    TileType::StoW => {
                        return Option::Some((DIRECTION_WEST, (self.pos.0 - 1, self.pos.1)));
                    },
                    _ => Option::None
                }
            },
            DIRECTION_SOUTH => // from north,
            {
                match self.tile_type {
                    TileType::NtoE => {
                        return Option::Some((DIRECTION_EAST, (self.pos.0 + 1, self.pos.1)));
                    },
                    TileType::Vert => {
                        return Option::Some((DIRECTION_SOUTH, (self.pos.0, self.pos.1 - 1)));
                    },
                    TileType::NtoW => {
                        return Option::Some((DIRECTION_WEST, (self.pos.0 - 1, self.pos.1)));
                    },
                    _ => Option::None
                }
            },
            _ => panic!("Didn't expect direction {:?}", abs_entry_direction)
        }
    }
}

impl Default for TileType {
    fn default() -> Self {
        Self::Ground
    }
}

// ----------------------------------------------------
// Model
// ----------------------------------------------------

pub const DIRECTION_NORTH: Direction = (0,1);
pub const DIRECTION_SOUTH: Direction = (0,-1);
pub const DIRECTION_WEST: Direction = (-1,0);
pub const DIRECTION_EAST: Direction = (1,0);

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Field {
    pub tiles: Vec<Vec<Tile>>,
    pub animal_pos: Position
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Tile {
    pub tile_type: TileType,
    pub pos: Position
}

#[derive(Debug, Clone, PartialEq)]
pub enum TileType{
    Vert,
    Horiz,
    NtoE,
    NtoW,
    StoW,
    StoE,
    Ground,
    Animal
}

pub type Direction = (isize, isize);
pub type Position = (isize, isize);

// ----------------------------------------------------
// readers
// ----------------------------------------------------

pub fn read_tile(in_char: &char, in_pos: Position) -> Tile
{
    Tile 
    { 
        tile_type: match in_char 
        {
            '|' => TileType::Vert,
            '-' => TileType::Horiz,
            'L' => TileType::NtoE,
            'J' => TileType::NtoW,
            '7' => TileType::StoW,
            'F' => TileType::StoE,
            '.' => TileType::Ground,
            'S' => TileType::Animal,
            c => panic!("Didn't expect other char: {}", c)
        }, 
        pos: in_pos
    }
}

pub fn read_field(in_path: &Path) -> Field
{
    let lines = BufReader::new(
        File::open(in_path).expect("Input file does not open")
     )
        .lines()
        .map(Result::unwrap)
        .enumerate()
        .map(|(y, line)|
        {
            line
                .char_indices()
                .map(|(x, c)| 
                    read_tile(
                        &c, 
                        (x.try_into().unwrap(), y.try_into().unwrap())
                    )
                )
                .collect()
        })
        .collect::<Vec<Vec<Tile>>>();

    let animal_pos = lines.iter()
        .enumerate()
        .find_map(|(y, row)|
            row.iter()
                .position(|tile| tile.tile_type == TileType::Animal)
                .map(|x| (x as isize, y as isize))
        )
        .unwrap();

    Field { tiles: lines, animal_pos: animal_pos }
}