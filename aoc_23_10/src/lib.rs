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
    println!(
        "Steps to farthest tile from animal: {}",
        read_field(Path::new("./input/aoc_23_10/input.txt")).generate_pipeloop().steps_to_farthest_point
    );
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
        let x = pos.1 as usize;
        let y = pos.0 as usize;
        &self.tiles[x][y]
    }

    pub fn max_x(&self) -> usize {
        self.tiles[0].len() - 1
    }

    pub fn max_y(&self) -> usize {
        self.tiles.len() - 1
    }

    pub fn max_xi(&self) -> isize {
        (self.tiles[0].len() - 1) as isize
    }

    pub fn max_yi(&self) -> isize {
        (self.tiles.len() - 1) as isize
    }

    pub fn exceeds_bounds(&self, pos: Position) -> bool
    {
        pos.0 < 0 
        || pos.1 < 0 
        || pos.0 > self.max_xi()
        || pos.1 > self.max_yi()
    }

    pub fn animal_tile(&self) -> &Tile {
        &self.at(self.animal_pos)
    }

    pub fn connecting_tiles(&self, tile : &Tile) -> Vec<&Tile> {
        let mut connected_tiles = vec![];

        let pos_east = (tile.pos.0 + 1, tile.pos.1);
        let pos_west = (tile.pos.0 - 1, tile.pos.1);
        let pos_south = (tile.pos.0, tile.pos.1 + 1);
        let pos_north = (tile.pos.0, tile.pos.1 - 1);

        if tile.connects_east() && !self.exceeds_bounds(pos_east) && self.at(pos_east).connects_west() {
            connected_tiles.push(self.at(pos_east));
        }
        if tile.connects_west() && !self.exceeds_bounds(pos_west) && self.at(pos_west).connects_east() {
            connected_tiles.push(self.at(pos_west));
        }
        if tile.connects_south() && !self.exceeds_bounds(pos_south) && self.at(pos_south).connects_north() {
            connected_tiles.push(self.at(pos_south));
        }
        if tile.connects_north() && !self.exceeds_bounds(pos_north) && self.at(pos_north).connects_south() {
            connected_tiles.push(self.at(pos_north));
        }

        connected_tiles
    }

    pub fn generate_pipeloop(&self) -> PipeLoop
    {
        let mut steps = 0;

        let mut previous_tiles = vec![ self.animal_tile() ];
        let mut current_tiles = self.connecting_tiles(self.animal_tile());
        let mut next_tiles = vec![];

        let mut trace_a = vec![ current_tiles[0] ];
        let mut trace_b = vec![ current_tiles[1] ];

        loop 
        {
            for tile in &current_tiles
            {
                for direction in tile.enterable_directions() {
                    if let Some(next_pos) = tile.pipe_forward(direction) {
                        if self.exceeds_bounds(next_pos) {
                            continue;
                        }
                        let next_tile = self.at(next_pos);

                        if previous_tiles.contains(&next_tile) || next_tiles.contains(&next_tile) {
                            continue;
                        }
                        else {
                            next_tiles.push(next_tile);
                            if trace_a.contains(tile) {
                                trace_a.push(next_tile);
                            }
                            if trace_b.contains(tile) {
                                trace_b.push(next_tile);
                            }
                        }
                    }
                }
            }

            previous_tiles = current_tiles;
            current_tiles = next_tiles.clone();
            next_tiles.clear();

            steps += 1;

            if current_tiles.is_empty() {
                break;
            }
        }

        let trace_a_copy: Vec<Tile> = trace_a.into_iter().cloned().collect();
        let trace_b_copy: Vec<Tile> = trace_b.into_iter().cloned().collect();

        PipeLoop{ trace_a: trace_a_copy, trace_b: trace_b_copy, steps_to_farthest_point: steps }
    }
}

impl PipeLoop {
    pub fn is_on_loop(&self, tile: &Tile) -> bool {
        self.trace_a.contains(tile) || self.trace_b.contains(tile)
    }

    pub fn expand_to_area(&self, initial_tile: &Tile, field: &Field) -> Area {

        let mut tiles: Vec<Tile> = vec![];
        let mut unexplored_tiles: Vec<Tile> = vec![ initial_tile.clone() ];

        let mut is_border_area = false;

        let mut is_valid_neighbour = |pos, tiles_: &Vec<Tile>, unexplored_tiles_: &Vec<Tile>| {
            let (x,y) = pos;
            if field.exceeds_bounds(pos) {
                is_border_area = true;
                return None;
            }
            let tile = field.at((x,y));
            if self.is_on_loop(tile) {
                return None;
            }
            if tiles_.contains(&tile) || unexplored_tiles_.contains(&tile) {
                return None;
            }
            Some(tile.clone())
        };

        loop {
            let inspected_tile_option = unexplored_tiles.pop();
            if inspected_tile_option.is_none() {
                break;
            }

            let inspected_tile = inspected_tile_option.unwrap();
            if inspected_tile.tile_type == TileType::Ground {
                tiles.push(inspected_tile.clone());
            }

            let (x,y) = inspected_tile.pos;
            if let Some(tile) = is_valid_neighbour((x, y-1), &tiles, &unexplored_tiles) {
                unexplored_tiles.push(tile.clone());
            }
            if let Some(tile) = is_valid_neighbour((x, y+1), &tiles, &unexplored_tiles) {
                unexplored_tiles.push(tile.clone());
            }
            if let Some(tile) = is_valid_neighbour((x-1, y), &tiles, &unexplored_tiles) {
                unexplored_tiles.push(tile.clone());
            }
            if let Some(tile) = is_valid_neighbour((x+1, y), &tiles, &unexplored_tiles) {
                unexplored_tiles.push(tile.clone());
            }
        }

        Area { tiles: tiles, is_border_area: is_border_area }
    }

    pub fn num_enclosed_areas(&self, _: &Field) -> usize {

        // let border_connected_tiles: Vec<&Tile> = vec![];

        // let mut pipe: Vec<Tile> = self.trace_a.clone();
        // pipe.append(&mut self.trace_b.clone());

        // for pipe_tile in pipe {
            
        // }

        0
    }
}

impl Tile {
    pub fn connects_north(&self) -> bool {
        [ TileType::Animal, TileType::NtoW, TileType::Vert, TileType::NtoE ].contains(&self.tile_type)
    }
    pub fn connects_south(&self) -> bool {
        [ TileType::Animal, TileType::StoW, TileType::Vert, TileType::StoE ].contains(&self.tile_type)
    }
    pub fn connects_west(&self) -> bool {
        [ TileType::Animal, TileType::NtoW, TileType::Horiz, TileType::StoW ].contains(&self.tile_type)
    }
    pub fn connects_east(&self) -> bool {
        [ TileType::Animal, TileType::NtoE, TileType::Horiz, TileType::StoE ].contains(&self.tile_type)
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

    pub fn pipe_forward(&self, abs_entry_direction: Direction) -> Option<Position>
    {
        match abs_entry_direction {
            DIRECTION_WEST =>
            {
                match self.tile_type {
                    TileType::NtoW => {
                        return Option::Some((self.pos.0, self.pos.1 - 1));
                    },
                    TileType::Horiz => {
                        return Option::Some((self.pos.0 + 1, self.pos.1));
                    },
                    TileType::StoW => {
                        return Option::Some((self.pos.0, self.pos.1 + 1));
                    },
                    _ => Option::None
                }
            },
            DIRECTION_EAST =>
            {
                match self.tile_type {
                    TileType::NtoE => {
                        return Option::Some((self.pos.0, self.pos.1 - 1));
                    },
                    TileType::Horiz => {
                        return Option::Some((self.pos.0 - 1, self.pos.1));
                    },
                    TileType::StoE => {
                        return Option::Some((self.pos.0, self.pos.1 + 1));
                    },
                    _ => Option::None
                }
            },
            DIRECTION_SOUTH =>
            {
                match self.tile_type {
                    TileType::StoE => {
                        return Option::Some((self.pos.0 + 1, self.pos.1));
                    },
                    TileType::Vert => {
                        return Option::Some((self.pos.0, self.pos.1 - 1));
                    },
                    TileType::StoW => {
                        return Option::Some((self.pos.0 - 1, self.pos.1));
                    },
                    _ => Option::None
                }
            },
            DIRECTION_NORTH =>
            {
                match self.tile_type {
                    TileType::NtoE => {
                        return Option::Some((self.pos.0 + 1, self.pos.1));
                    },
                    TileType::Vert => {
                        return Option::Some((self.pos.0, self.pos.1 + 1));
                    },
                    TileType::NtoW => {
                        return Option::Some((self.pos.0 - 1, self.pos.1));
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

pub const DIRECTION_NORTH: Direction = (0,-1);
pub const DIRECTION_SOUTH: Direction = (0,1);
pub const DIRECTION_WEST: Direction = (-1,0);
pub const DIRECTION_EAST: Direction = (1,0);

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Field {
    pub tiles: Vec<Vec<Tile>>,
    pub animal_pos: Position
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PipeLoop {
    pub trace_a: Vec<Tile>,
    pub trace_b: Vec<Tile>,
    pub steps_to_farthest_point : usize
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Area {
    pub tiles: Vec<Tile>,
    pub is_border_area: bool
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Tile {
    pub tile_type: TileType,
    pub pos: Position
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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