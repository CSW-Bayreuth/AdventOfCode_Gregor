// ----------------------------------------------------
// ----------------------------------------------------
// imports
// ----------------------------------------------------
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path, collections::HashMap
};
use rayon::prelude::*;

// ----------------------------------------------------
// App
// ----------------------------------------------------
pub fn start_app() {
    println!(
        "Lowest location number that corresponds to any of the initial seed numbers: {}",
        read_almanac(Path::new("./input/aoc_23_05/input.txt")).lowest_location_for_seeds_to_plant()
    );

    println!(
        "Lowest location number that corresponds to any of the initial seed ranges: {}",
        read_almanac(Path::new("./input/aoc_23_05/input.txt")).lowest_location_for_seed_ranges_to_plant_par()
    );
}

// ----------------------------------------------------
// Evaluation
// ----------------------------------------------------

// ----------------------------------------------------
// Model
// ----------------------------------------------------

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Almanac {
    pub seeds_to_plant: Vec<usize>,

    pub category_count: usize,
    pub map_collections: Vec<CategoryMapCollection>,
}

impl Almanac {
    pub fn lowest_location_for_seeds_to_plant(&self) -> usize 
    {
        let mut lowest_loc = usize::MAX;

        for seed in self.seeds_to_plant.iter()
        {
            let mut current_num = *seed;

            for map_collection in self.map_collections.iter()
            {
                current_num = map_collection.src_to_dest(current_num);
            }

            if current_num < lowest_loc {
                lowest_loc = current_num;
            }
        }

        lowest_loc
    }
    
    pub fn lowest_location_for_seed_ranges_to_plant(&mut self) -> usize 
    {
        let mut lowest_loc = usize::MAX;

        //(0..self.seeds_to_plant.len()).into_par_iter().map(|index|
        for index in 0..self.seeds_to_plant.len()
        {
            if index % 2 == 0
            {
                let seed_start = self.seeds_to_plant[index];
                let seed_end = seed_start + self.seeds_to_plant[index+1];

                println!("Range {}/{}, with len {}...", index/2+1, self.seeds_to_plant.len()/2, seed_end-seed_start);

                //let local_lowest_loc = (seed_start..seed_end).into_par_iter().map(|seed|
                for seed in seed_start..seed_end
                {
                    // go through all categories
                    let mut current_num = seed;
                    for map_collection in self.map_collections.iter()
                    {
                        current_num = map_collection.src_to_dest_with_cache(current_num);
                    }
                    let local_low = current_num;

                    // propagate lowest loc up and cache it
                    let mut current_num = seed;
                    for map_collection in self.map_collections.iter_mut()
                    {
                        if map_collection.min_loc_cache.contains_key(&current_num) {
                            if map_collection.min_loc_cache[&current_num] > local_low {
                                map_collection.min_loc_cache.insert(current_num, local_low);
                            }
                        }
                        else {
                            map_collection.min_loc_cache.insert(current_num, local_low);
                        }
                        current_num = map_collection.src_to_dest(current_num);
                    }
        
                    if local_low < lowest_loc {
                        lowest_loc = local_low;
                    }
                }

                println!("Range {}/{} with len {} done!", index/2+1, self.seeds_to_plant.len()/2, seed_end-seed_start);
            }
        }

        lowest_loc
    }
    
    pub fn lowest_location_for_seed_ranges_to_plant_par(&mut self) -> usize 
    {
        (0..self.seeds_to_plant.len()).into_par_iter().map(|index|
        //for index in 0..self.seeds_to_plant.len()
        {
            if index % 2 == 0
            {
                let seed_start = self.seeds_to_plant[index];
                let seed_end = seed_start + self.seeds_to_plant[index+1];

                println!("Range {}/{}, with len {}...", index/2+1, self.seeds_to_plant.len()/2, seed_end-seed_start);

                let res = (seed_start..seed_end).into_par_iter().map(|seed|
                //for seed in seed_start..seed_end
                {
                    // go through all categories
                    let mut current_num = seed;
                    for map_collection in self.map_collections.iter()
                    {
                        current_num = map_collection.src_to_dest(current_num);
                    }
                    
                    current_num
                }).min()
                .unwrap();

                println!("Range {}/{} with len {} done!", index/2+1, self.seeds_to_plant.len()/2, seed_end-seed_start);

                res
            }
            else {
                usize::MAX
            }
        }).min().unwrap()
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CategoryMapCollection {
    pub category: usize,
    pub maps: Vec<CategoryMap>,
    pub min_loc_cache: HashMap::<usize, usize>
}

impl CategoryMapCollection {
    pub fn src_to_dest(&self, src: usize) -> usize
    {
        let results = self.maps
            .iter()
            .filter_map(|map| map.src_to_dest(src))
            .collect::<Vec<usize>>();

        if results.is_empty() { src } else { results[0] }
    }
    
    pub fn src_to_dest_with_cache(&self, src: usize) -> usize
    {
        if self.min_loc_cache.contains_key(&src) { 
            self.min_loc_cache[&src] 
        }
        else {
            self.src_to_dest(src)
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CategoryMap {
    pub range_start_dest: usize,
    pub range_start_src: usize,
    pub range_len: usize,
}

impl CategoryMap {
    pub fn src_to_dest(&self, src: usize) -> Option<usize> 
    {
        if self.range_start_src <= src && src < self.range_start_src + self.range_len
        {
            Some(self.range_start_dest + (src - self.range_start_src))
        }
        else { None }
    }
}

// ----------------------------------------------------
// readers
// ----------------------------------------------------

pub fn read_almanac(in_path: &Path) -> Almanac
{
    let mut almanac = Almanac::default();

    let lines = BufReader::new(
        File::open(in_path).expect("Input file does not open")
     )
        .lines()
        .map(Result::unwrap);

    let mut cur_map_coll = CategoryMapCollection::default();
    let mut was_reading_map_collection = false;

    for line in lines 
    {
        match line.as_str() 
        {
            _ if line.as_str().starts_with("seeds") 
                => 
                {
                    almanac.seeds_to_plant = read_seeds(line.as_str());
                },

            "" => {},

            _ if line.contains("map")
                => 
                { 
                    if was_reading_map_collection
                    {
                        almanac.map_collections.push(cur_map_coll);
                    }
                    cur_map_coll = CategoryMapCollection::default();
                    cur_map_coll.category = almanac.map_collections.len();
                    was_reading_map_collection = true;
                },

            _ 
                => cur_map_coll.maps.push(read_category_map(line.as_str())),
        }
    }

    almanac.map_collections.push(cur_map_coll);

    almanac.category_count = almanac.map_collections.len();

    almanac
}

pub fn read_seeds(in_str: &str) -> Vec<usize>
{
    in_str
        .to_string()
        .split(" ")
        .map(str::parse)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect()
}

pub fn read_category_map(in_str: &str) -> CategoryMap
{
    let split = in_str
        .to_string()
        .split(" ")
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<usize>>();

    CategoryMap { range_start_dest: split[0], range_start_src: split[1], range_len: split[2] }
}
