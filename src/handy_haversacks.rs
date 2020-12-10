/******************************************************************************/
/* Advent of Code 2020 day 6:                                                 */
/*   -- Handy Haversacks --                                                   */
/******************************************************************************/
/******************************************************************************/
/* Dependencies                                                               */
/******************************************************************************/
extern crate petgraph;
extern crate regex;

use core::cmp::Ordering;
use petgraph::Graph;
use petgraph::algo::has_path_connecting;
use petgraph::graph::NodeIndex;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/******************************************************************************/
/* Constant definitions                                                       */
/******************************************************************************/
// MY_BAG_COLOR: the color of your bag. The goal of the program is to determine
//             how many bag colors could eventually contain your bag color
const MY_BAG_COLOR: &str = "shiny gold";

/******************************************************************************/
/* Structure definitions                                                      */
/******************************************************************************/
// BagChild: A "shallow copy" of a "child color" bag which is contained by
//   a bag of "parent color".
#[derive(Clone, Debug)]
struct BagChild {
    color: String,      // color of the child bag
    count: u32,         // number of child-color bags contained in the parent bag
}

// "Parent bag" which contains a number of other bags.
#[derive(Debug)]
struct Bag {
    color: String,
    contents: Vec<BagChild>,
}

// Ord implementation for sorting
// Sort by length of the "contents" vec.
impl Ord for Bag {
    fn cmp(&self, other: &Self) -> Ordering {
        self.contents.len().cmp(&other.contents.len())
    }
}

impl PartialOrd for Bag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
    }
}

impl Eq for Bag { }

/******************************************************************************/
/* Subroutines                                                                */
/******************************************************************************/
// Parse the input file and build a set of "shallow copies" of bags. These bags
//   are only one level deep.
fn parse_input(input: &str) -> Result<Vec<Bag>, Box<dyn Error>> {
    let file = File::open(input)?;
    
    let contained_bag_regex = Regex::new(r"\sbag(s?)(,|\.)(\s?)")?;

    // construct data structure from lines
    let mut bags: Vec<Bag> = Vec::new();
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let mut contained_bags: Vec<BagChild> = Vec::new();
    while let Ok(line_length) = reader.read_line(&mut line) {
        match line_length {
            // if line_length is 0, we've reached end of file
            0 => break,
            _ => {
                let split_input_line: Vec<&str> = line.split(" bags contain ").collect();
                
                // first string in input line is color of the bag
                let color = split_input_line.get(0).unwrap();

                let contained_bag_list = split_input_line.get(1).unwrap();
                for contained_bag_str in contained_bag_regex.split(contained_bag_list) {
                    // it's possible that a bag contains no other bags
                    // skip this bag if this is the case
                    if !contained_bag_str.trim().is_empty() && !contained_bag_str.contains("no other") {
                        // split contained_bag_str into number and color
                        // split the string on the first space.
                        // number comes first, then color
                        let mut splitter = contained_bag_str.splitn(2, ' ');
                            
                        let contained_num = splitter.next().unwrap().parse()?;
        
                        let contained_color = splitter.next().unwrap();
        
                        contained_bags.push(BagChild {
                            color: contained_color.to_string(),
                            count: contained_num,
                        });
                    }
                }

                bags.push(Bag { 
                    color: color.to_string(),
                    contents: contained_bags.clone(),
                });

                contained_bags.clear();
                line.clear();   
            }
        }
    }
    
    Ok(bags)
}

/******************************************************************************/
/* build_bag_tree: build a tree of bags, where each bag has a number of       */
/*   children                                                                 */
/******************************************************************************/
fn build_bag_tree(rules: &Vec<Bag>) -> (Graph<String, u32>, HashMap<String, NodeIndex>) {
    let mut bag_graph: Graph<String, u32> = Graph::new();
    // construct a map of color names to NodeIndexes
    let mut bag_nodes: HashMap<String, NodeIndex> = HashMap::new();

    // build the graph's nodes
    for bag in rules.iter() {
        bag_nodes.insert(bag.color.clone(), bag_graph.add_node(bag.color.clone()));
    }

    // build graph's edges
    for parent_bag in rules.iter() {
        for child_bag in parent_bag.contents.iter() {
            let parent_node = bag_nodes.get(&parent_bag.color).unwrap();
            let child_node = bag_nodes.get(&child_bag.color).unwrap();
            
            // use child bag's count as the weight
            bag_graph.update_edge(*parent_node, *child_node, child_bag.count);
        }
    }
    
    return (bag_graph, bag_nodes);
}

// recursively get the number of child (grandchild, etc) bags contained in the passed-in bag node
fn get_child_bag_count(bag_graph: &Graph<String, u32>, my_bag_node: &NodeIndex) -> u32 {
    println!("Recursing");
    let mut count = 1;

    let neighbors = bag_graph.neighbors(*my_bag_node);
    for neighbor in neighbors {
        let edge = bag_graph.find_edge(*my_bag_node, neighbor).unwrap();
        count += bag_graph.edge_weight(edge).unwrap() * get_child_bag_count(bag_graph, &neighbor);
    }
    
    return count;
}


/******************************************************************************/
/* Main routine                                                               */
/******************************************************************************/
pub fn run(input: &str) {
    // parse input file and build a list of "rules" about bags
    let bag_index: Vec<Bag> = parse_input(input).unwrap();

    // build a tree of bags which contain other bags
    // the weight (u32) value is the number of child bags that each bag contains
    let bag_graph_rtn = build_bag_tree(&bag_index);
    let bag_graph = bag_graph_rtn.0;
    let bag_nodes = bag_graph_rtn.1;

    let mut paths_to_goal = 0;
    let my_bag_node = bag_nodes.get(MY_BAG_COLOR).unwrap();

    for start_node in bag_nodes.values() {
        // don't count the goal -> goal case as a path
        if start_node == my_bag_node {
            continue;
        }
        if has_path_connecting(&bag_graph, *start_node, *my_bag_node, None) {
            paths_to_goal += 1;
        }
    }
    println!("Paths to goal (part 1 solution): {}", paths_to_goal);

    // perform a depth-first search from my bag to get the total number of bags
    //  that must be inside it
    // have to subtract one from the result; don't count my own bag
    let my_bag_contents = get_child_bag_count(&bag_graph, my_bag_node) - 1;
    println!("Number of bags inside my bag (part 2): {}", my_bag_contents);
}