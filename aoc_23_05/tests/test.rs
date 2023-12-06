#![allow(dead_code)]
#![allow(non_snake_case)]
#![cfg(test)]
use std::collections::HashMap;

// ----------------------------------------------------
// imports
// ----------------------------------------------------
use rstest::rstest;

use aoc_23_05::*;

// ----------------------------------------------------
// tests funcs - evaluation
// ----------------------------------------------------

#[rstest]
#[case(std::path::Path::new("./../input/aoc_23_05/input_example.txt"), 46)]
fn test_lowest_location_for_seed_ranges_to_plant(
    #[case] in_path: &std::path::Path,
    #[case] expected: usize
) {
    assert_eq!(read_almanac(in_path).lowest_location_for_seed_ranges_to_plant_par(), expected);
}

#[rstest]
#[case(std::path::Path::new("./../input/aoc_23_05/input_example.txt"), 35)]
fn test_lowest_location_for_seeds_to_plant(
    #[case] in_path: &std::path::Path,
    #[case] expected: usize
) {
    assert_eq!(read_almanac(in_path).lowest_location_for_seeds_to_plant(), expected);
}

#[rstest]
#[case(
    CategoryMapCollection{ category: 0, maps: vec![
            CategoryMap{
                range_start_dest: 50,
                range_start_src: 98,
                range_len: 2, },
            CategoryMap{
                range_start_dest: 52,
                range_start_src: 50,
                range_len: 48, }
        ], 
        min_loc_cache: HashMap::new()
    }, 
    50, 52
)]
#[case(
    CategoryMapCollection{ category: 0, maps: vec![
            CategoryMap{
                range_start_dest: 50,
                range_start_src: 98,
                range_len: 2, },
            CategoryMap{
                range_start_dest: 52,
                range_start_src: 50,
                range_len: 48, }
        ], 
        min_loc_cache: HashMap::new()
    }, 98, 50
)]
#[case(
    CategoryMapCollection{ category: 0, maps: vec![
            CategoryMap{
                range_start_dest: 50,
                range_start_src: 98,
                range_len: 2},
            CategoryMap{
                range_start_dest: 52,
                range_start_src: 50,
                range_len: 48, }
        ], 
        min_loc_cache: HashMap::new()
    }, 77, 79
)]
#[case(
    CategoryMapCollection{ category: 0, maps: vec![
            CategoryMap{
                range_start_dest: 50,
                range_start_src: 98,
                range_len: 2, },
            CategoryMap{
                range_start_dest: 52,
                range_start_src: 50,
                range_len: 48, }
        ], 
        min_loc_cache: HashMap::new()
    }, 
    1000, 1000
)]
fn test_categorymapcoll_src_to_dest(
    #[case] in_mapcoll: CategoryMapCollection,
    #[case] in_src: usize,
    #[case] expected: usize
) {
    assert_eq!(in_mapcoll.src_to_dest(in_src), expected);
}

#[rstest]
#[case(CategoryMap { range_start_dest: 50, range_start_src: 98, range_len: 2 }, 97, None)]
#[case(CategoryMap { range_start_dest: 50, range_start_src: 98, range_len: 2 }, 98, Some(50))]
#[case(CategoryMap { range_start_dest: 50, range_start_src: 98, range_len: 2 }, 99, Some(51))]
#[case(CategoryMap { range_start_dest: 50, range_start_src: 98, range_len: 2 }, 100, None)]
fn test_categorymap_src_to_dest(
    #[case] in_map: CategoryMap,
    #[case] in_src: usize,
    #[case] expected: Option<usize>
) {
    assert_eq!(in_map.src_to_dest(in_src), expected);
}

// ----------------------------------------------------
// tests funcs - reader
// ----------------------------------------------------

#[rstest]
#[case(
    std::path::Path::new("./../input/aoc_23_05/input_example2.txt"), 
    Almanac { 
        seeds_to_plant: vec![79, 14, 55, 13], 
        category_count: 2, 
        map_collections: vec![
            CategoryMapCollection{
                category: 0,
                maps: vec![
                    CategoryMap{
                        range_start_dest: 50,
                        range_start_src: 98,
                        range_len: 2,
                    },
                    CategoryMap{
                        range_start_dest: 52,
                        range_start_src: 50,
                        range_len: 48,
                    }
                ], 
                min_loc_cache: HashMap::new()
            },
            CategoryMapCollection{
                category: 1,
                maps: vec![
                    CategoryMap{
                        range_start_dest: 0,
                        range_start_src: 15,
                        range_len: 37,
                    }
                ], 
                min_loc_cache: HashMap::new()
            }
        ] 
    }
)]
fn test_read_almanac(
    #[case] in_path: &std::path::Path,
    #[case] expected: Almanac
) {
    assert_eq!(read_almanac(in_path), expected);
}

#[rstest]
#[case("seeds: 79 14 55 13", vec![79, 14, 55, 13])]
fn test_read_category(
    #[case] in_str: &str,
    #[case] expected: Vec<usize>
) {
    assert_eq!(read_seeds(in_str), expected);
}

#[rstest]
#[case("50 98 2", CategoryMap { range_start_dest: 50, range_start_src: 98, range_len: 2 })]
fn test_read_category_map(
    #[case] in_str: &str,
    #[case] expected: CategoryMap
) {
    assert_eq!(read_category_map(in_str), expected);
}

// ----------------------------------------------------
// tests funcs - readers
// ----------------------------------------------------
