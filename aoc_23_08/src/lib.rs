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
        read_plan(Path::new("./input/aoc_23_08/input.txt")).steps_to_reach_end_ghost2()
    );
}

// ----------------------------------------------------
// Evaluation
// ----------------------------------------------------

impl Plan {
    
    pub fn steps_to_reach_end_ghost2(&self) -> usize
    {
        let mut cur_node_ids = self.nodeid_to_node
            .keys()
            .filter(|id| id._ends_with_a)
            .collect::<Vec<&NodeId>>();

        let mut steps_till_unique_z: Vec<usize> = vec![];
        steps_till_unique_z.reserve(cur_node_ids.len());

        let mut instr_index = 0;

        let mut steps = 0;

        loop 
        {
            if cur_node_ids.is_empty()
            {
                break;
            }

            let mut next_node_ids = vec![];
            for node_id in cur_node_ids
            {
                let cur_node = self.nodeid_to_node.get(node_id).unwrap();
                let cur_instr = self.nav_instructions[instr_index];

                if node_id._ends_with_z
                {
                    steps_till_unique_z.push(steps);
                }
                else 
                {
                    next_node_ids.push(if cur_instr { &cur_node.left } else { &cur_node.right });
                }
            }
            cur_node_ids = next_node_ids;

            steps += 1;

            instr_index = (instr_index + 1) % self.nav_instructions.len();
        }

        return steps_till_unique_z
            .iter()
            .fold(1, |acc, n| {
                let n_: usize = *n;
                num::integer::lcm(acc, n_)
            })
    }


    
    pub fn steps_to_reach_end_ghost(&self) -> u128
    {
        let mut cur_node_ids = self.nodeid_to_node
            .keys()
            .filter(|id| id._ends_with_a)
            .collect::<Vec<&NodeId>>();

        let mut instr_index = 0;

        let mut steps = 0;

        let mut max_count_so_far: u128 = 0;

        loop 
        {
            if cur_node_ids.iter().all(|id| id._ends_with_z)
            {
                break;
            }

            max_count_so_far = u128::max(max_count_so_far, cur_node_ids.iter().filter(|id| id._ends_with_z).count().try_into().unwrap());

            if steps % 1000000 == 0 {
                println!("{} {}", steps, max_count_so_far);
            }

            let mut next_node_ids = vec![];
            for node_id in cur_node_ids
            {
                let cur_node = self.nodeid_to_node.get(node_id).unwrap();
                let cur_instr = self.nav_instructions[instr_index];
    
                next_node_ids.push(if cur_instr { &cur_node.left } else { &cur_node.right });
            }
            cur_node_ids = next_node_ids;

            steps += 1;

            instr_index = (instr_index + 1) % self.nav_instructions.len();
        }

        return steps;
    }
    
    pub fn steps_to_reach_end_from_aaa(&self) -> usize
    {
        self.steps_to_reach_end_generic(NodeId::from_str("AAA"))
    }

    pub fn steps_to_reach_end_generic(&self, start_node: NodeId) -> usize
    {
        let mut instr_index = 0;

        let mut steps = 0;

        let mut cur_node_id = &start_node;

        loop 
        {
            if *cur_node_id == NodeId::from_str("ZZZ")
            {
                break;
            }

            let cur_node = self.nodeid_to_node.get(&cur_node_id).unwrap();
            let cur_instr = self.nav_instructions[instr_index];

            cur_node_id = if cur_instr {&cur_node.left} else {&cur_node.right};

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

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct NodeId {
    pub str: String,
    pub _ends_with_a: bool,
    pub _ends_with_z: bool,
}

impl NodeId {
    pub fn from_str(in_str: &str) -> NodeId
    {
        NodeId { 
            str: in_str.to_string(),
            _ends_with_a: in_str.ends_with("A"),
            _ends_with_z: in_str.ends_with("Z")
        }
    }
}

pub type Instruction = bool;

// ----------------------------------------------------
// readers
// ----------------------------------------------------

pub fn read_instructions(in_str: &str) -> Vec<Instruction>
{
    in_str.chars().map(|c| if c == 'L' {true} else {false}).collect()
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

    Node { 
        id: NodeId::from_str(ids[0]), 
        left: NodeId::from_str(ids[1]), 
        right: NodeId::from_str(ids[2]) 
    }
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
