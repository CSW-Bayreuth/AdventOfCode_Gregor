#![allow(dead_code)]
#![allow(non_snake_case)]
#![cfg(test)]

use std::path::Path;

// ----------------------------------------------------
// imports
// ----------------------------------------------------
use rstest::rstest;

use aoc_23_10::*;

// ----------------------------------------------------
// tests funcs - evaluation
// ----------------------------------------------------

#[rstest]
#[case(Tile{ tile_type: TileType::Horiz, pos: (5,5) }, DIRECTION_WEST, Some((6,5)))]
#[case(Tile{ tile_type: TileType::Horiz, pos: (5,5) }, DIRECTION_EAST, Some((4,5)))]

#[case(Tile{ tile_type: TileType::Vert, pos: (5,5) }, DIRECTION_NORTH, Some((5,6)))]
#[case(Tile{ tile_type: TileType::Vert, pos: (5,5) }, DIRECTION_SOUTH, Some((5,4)))]

#[case(Tile{ tile_type: TileType::NtoE, pos: (5,5) }, DIRECTION_NORTH, Some((6,5)))]
#[case(Tile{ tile_type: TileType::NtoE, pos: (5,5) }, DIRECTION_EAST, Some((5,4)))]

#[case(Tile{ tile_type: TileType::NtoW, pos: (5,5) }, DIRECTION_NORTH, Some((4,5)))]
#[case(Tile{ tile_type: TileType::NtoW, pos: (5,5) }, DIRECTION_WEST, Some((5,4)))]

#[case(Tile{ tile_type: TileType::StoE, pos: (5,5) }, DIRECTION_SOUTH, Some((6,5)))]
#[case(Tile{ tile_type: TileType::StoE, pos: (5,5) }, DIRECTION_EAST, Some((5,6)))]

#[case(Tile{ tile_type: TileType::StoW, pos: (5,5) }, DIRECTION_SOUTH, Some((4,5)))]
#[case(Tile{ tile_type: TileType::StoW, pos: (5,5) }, DIRECTION_WEST, Some((5,6)))]

#[case(Tile{ tile_type: TileType::StoW, pos: (5,5) }, DIRECTION_EAST, None)]
fn test_tile_pipe_forward(
    #[case] in_tile: Tile,
    #[case] in_entry_direction: Direction,
    #[case] expected: Option<Position>
)
{
    assert_eq!(in_tile.pipe_forward(in_entry_direction), expected);
}

#[rstest]
#[case(Path::new("./../input/aoc_23_10/input_example.txt"), 4)]
#[case(Path::new("./../input/aoc_23_10/input_example2.txt"), 8)]
fn test_steps_to_farthest_tile(
    #[case] in_path: &Path,
    #[case] expected: usize,
)
{
    assert_eq!(read_field(in_path).steps_to_farthest_tile(), expected);
}

#[rstest]
#[case(Path::new("./../input/aoc_23_10/input_example.txt"), 
    Tile{tile_type: TileType::Animal, pos: (1,1)}, 
    vec![
        Tile{tile_type: TileType::Horiz, pos: (2,1)},
        Tile{tile_type: TileType::Vert, pos: (1,2)},
    ]
)]
fn test_connecting_tiles(
    #[case] in_path: &Path,
    #[case] in_tile: Tile,
    #[case] expected: Vec<Tile>,
)
{
    let field = read_field(in_path);
    let solution = field.connecting_tiles(&in_tile);
    let solution_cleaned: Vec<Tile> = solution.into_iter().cloned().collect();
    assert_eq!(solution_cleaned, expected);
}

// ----------------------------------------------------
// tests funcs - readers
// ----------------------------------------------------

#[rstest]
#[case(Path::new("./../input/aoc_23_10/input_example.txt"),
Field { tiles: vec![
    vec![
        Tile{tile_type:TileType::Ground, pos:(0,0)}, 
        Tile{tile_type:TileType::Ground, pos:(1,0)}, 
        Tile{tile_type:TileType::Ground, pos:(2,0)}, 
        Tile{tile_type:TileType::Ground, pos:(3,0)}, 
        Tile{tile_type:TileType::Ground, pos:(4,0)}, ],
    vec![
        Tile{tile_type:TileType::Ground, pos:(0,1)}, 
        Tile{tile_type:TileType::Animal, pos:(1,1)}, 
        Tile{tile_type:TileType::Horiz, pos:(2,1)}, 
        Tile{tile_type:TileType::StoW, pos:(3,1)}, 
        Tile{tile_type:TileType::Ground, pos:(4,1)}, ],
    vec![
        Tile{tile_type:TileType::Ground, pos:(0,2)}, 
        Tile{tile_type:TileType::Vert, pos:(1,2)}, 
        Tile{tile_type:TileType::Ground, pos:(2,2)}, 
        Tile{tile_type:TileType::Vert, pos:(3,2)}, 
        Tile{tile_type:TileType::Ground, pos:(4,2)}, ],
    vec![
        Tile{tile_type:TileType::Ground, pos:(0,3)}, 
        Tile{tile_type:TileType::NtoE, pos:(1,3)}, 
        Tile{tile_type:TileType::Horiz, pos:(2,3)}, 
        Tile{tile_type:TileType::NtoW, pos:(3,3)}, 
        Tile{tile_type:TileType::Ground, pos:(4,3)}, ],
    vec![
        Tile{tile_type:TileType::Ground, pos:(0,4)}, 
        Tile{tile_type:TileType::Ground, pos:(1,4)}, 
        Tile{tile_type:TileType::Ground, pos:(2,4)}, 
        Tile{tile_type:TileType::Ground, pos:(3,4)}, 
        Tile{tile_type:TileType::Ground, pos:(4,4)}, ],], 
    animal_pos: (1,1) })]
fn test_read_field(
    #[case] in_path: &Path,
    #[case] expected: Field
)
{
    assert_eq!(read_field(in_path), expected);
}
