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
#[case(Path::new("./../input/aoc_23_10/input_example3.txt"), 8)]
#[case(Path::new("./../input/aoc_23_10/input.txt"), 6786)]
fn test_steps_to_farthest_tile(
    #[case] in_path: &Path,
    #[case] expected: usize,
)
{
    assert_eq!(read_field(in_path).generate_pipeloop().steps_to_farthest_point, expected);
}

#[rstest]
#[case(Path::new("./../input/aoc_23_10/input_example.txt"), 
    Tile{tile_type: TileType::Animal, pos: (1,1)}, 
    vec![
        Tile{tile_type: TileType::Horiz, pos: (2,1)},
        Tile{tile_type: TileType::Vert, pos: (1,2)},
    ]
)]
#[case(Path::new("./../input/aoc_23_10/input.txt"), 
    Tile{tile_type: TileType::StoE, pos: (79,28)}, 
    vec![
        Tile{tile_type: TileType::NtoW, pos: (80,28)},
        Tile{tile_type: TileType::Vert, pos: (79,29)},
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

#[rstest]
#[case(Path::new("./../input/aoc_23_10/input_example4.txt"),
    Tile{tile_type: TileType::Ground, pos: (2,6)},
    Area{tiles: vec![ Tile{tile_type: TileType::Ground, pos: (2,6)}, Tile{tile_type: TileType::Ground, pos: (3,6)} ], is_border_area: false}
)]
fn test_expand_to_area(
    #[case] in_path: &Path,
    #[case] in_tile : Tile,
    #[case] expected : Area
)
{
    let field = read_field(in_path);
    let pipeloop = field.generate_pipeloop();
    assert_eq!(pipeloop.expand_to_area(&in_tile, &field), expected);
}

// #[rstest]
// #[case(Path::new("./../input/aoc_23_10/input_example4.txt"), 4)]
// #[case(Path::new("./../input/aoc_23_10/input_example5.txt"), 8)]
// #[case(Path::new("./../input/aoc_23_10/input_example6.txt"), 10)]
// fn test_num_enclosed_areas(
//     #[case] in_path: &Path,
//     #[case] expected : usize
// )
// {
//     let field = read_field(in_path);
//     let pipeloop = field.generate_pipeloop();
//     assert_eq!(pipeloop.num_enclosed_areas(&field), expected);
// }

#[rstest]
#[case(Path::new("./../input/aoc_23_10/input_example.txt"), 4)]
#[case(Path::new("./../input/aoc_23_10/input_example2.txt"), 4)]
#[case(Path::new("./../input/aoc_23_10/input_example3.txt"), 4)]
#[case(Path::new("./../input/aoc_23_10/input_example4.txt"), 8)]
#[case(Path::new("./../input/aoc_23_10/input_example5.txt"), 9)]
#[case(Path::new("./../input/aoc_23_10/input_example6.txt"), 9)]
fn test_max_y(
    #[case] in_path: &Path,
    #[case] expected : usize
)
{
    let field = read_field(in_path);
    assert_eq!(field.max_y(), expected);
}

#[rstest]
#[case(Path::new("./../input/aoc_23_10/input_example.txt"), 4)]
#[case(Path::new("./../input/aoc_23_10/input_example2.txt"), 4)]
#[case(Path::new("./../input/aoc_23_10/input_example3.txt"), 4)]
#[case(Path::new("./../input/aoc_23_10/input_example4.txt"), 10)]
#[case(Path::new("./../input/aoc_23_10/input_example5.txt"), 19)]
#[case(Path::new("./../input/aoc_23_10/input_example6.txt"), 19)]
fn test_max_x(
    #[case] in_path: &Path,
    #[case] expected : usize,
)
{
    let field = read_field(in_path);
    assert_eq!(field.max_x(), expected);
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
