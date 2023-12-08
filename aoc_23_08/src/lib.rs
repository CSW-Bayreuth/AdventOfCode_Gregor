// ----------------------------------------------------
// ----------------------------------------------------
// imports
// ----------------------------------------------------
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path, collections::HashMap
};
use lazy_static::lazy_static;
use regex::Regex;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator, IndexedParallelIterator};

// ----------------------------------------------------
// App
// ----------------------------------------------------
pub fn start_app() {
 println!(
        "Steps are required to reach ZZZ: {}",
        read_plan(Path::new("./input/aoc_23_08/input.txt")).steps_to_reach_end_from_aaa()
    );
 
    println!(
        "Steps are required to reach __Z-only nodes: {}",
        read_plan(Path::new("./input/aoc_23_08/input.txt")).steps_to_reach_end_ghost()
    );
}

// ----------------------------------------------------
// Evaluation
// ----------------------------------------------------

impl Plan {
    
    pub fn steps_to_reach_end_ghost(&self) -> usize
    {
        let mut cur_node_ids = self.nodeid_to_node
            .keys()
            .filter(|id| id.ends_with("A"))
            .collect::<Vec<&String>>();

        let mut instr_index = 0;

        let mut steps = 0;

        loop 
        {
            if cur_node_ids.iter().all(|id| id.ends_with("Z"))
            {
                break;
            }

            println!("{}/{}", cur_node_ids.iter().filter(|id| id.ends_with("Z")).count(), cur_node_ids.len());

            let mut next_node_ids = vec![];
            for node_id in cur_node_ids
            {
                let cur_node = self.nodeid_to_node.get(node_id).unwrap();
                let cur_instr = self.nav_instructions[instr_index];
    
                next_node_ids.push(if cur_instr == 'L' { &cur_node.left } else { &cur_node.right });
            }
            cur_node_ids = next_node_ids;

            steps += 1;

            instr_index = (instr_index + 1) % self.nav_instructions.len();
        }

        return steps;
    }
    
    pub fn steps_to_reach_end_from_aaa(&self) -> usize
    {
        let start_node = "AAA".to_string();
        self.steps_to_reach_end_generic(&start_node)
    }

    pub fn steps_to_reach_end_generic(&self, start_node: &NodeId) -> usize
    {
        let mut instr_index = 0;

        let mut steps = 0;

        let mut cur_node_id = start_node;

        loop 
        {
            if cur_node_id == "ZZZ"
            {
                break;
            }

            let cur_node = self.nodeid_to_node.get(cur_node_id).unwrap();
            let cur_instr = self.nav_instructions[instr_index];

            cur_node_id = if cur_instr == 'L' { &cur_node.left } else { &cur_node.right };

            steps += 1;

            instr_index = (instr_index + 1) % self.nav_instructions.len();
        }

        return steps;
    }
}

// ----------------------------------------------------
// Model
// ----------------------------------------------------

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Plan {
    pub nav_instructions: Vec<Instruction>,
    pub nodeid_to_node: HashMap<NodeId, Node>
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub left: NodeId,
    pub right: NodeId
}

pub type NodeId = String;

pub type Instruction = char;

// ----------------------------------------------------
// readers
// ----------------------------------------------------

pub fn read_instructions(in_str: &str) -> Vec<Instruction>
{
    in_str.chars().collect()
}

pub fn read_node(in_str: &str) -> Node
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[A-Z1-9]{3}").unwrap();
    }

    let ids = RE
        .find_iter(in_str)
        .map(|m| m.as_str())
        .collect::<Vec<&str>>();

    Node { id: ids[0].to_string(), left: ids[1].to_string(), right: ids[2].to_string() }
}

pub fn read_nodeid_to_node(in_lines: Vec<&str>) -> HashMap<NodeId, Node>
{
    in_lines
        .iter()
        .map(|l| read_node(*l))
        .map(|n| (n.id.clone(), n))
        .collect()
}

pub fn read_plan(in_path: &Path) -> Plan
{
    let lines = BufReader::new(
        File::open(in_path).expect("Input file does not open")
     )
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>();

    Plan { 
        nav_instructions: read_instructions(&lines[0]), 
        nodeid_to_node: read_nodeid_to_node(lines
            .iter()
            .map(String::as_str)
            .skip(2)
            .collect()) 
    }
}
