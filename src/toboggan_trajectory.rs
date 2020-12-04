use std::io::prelude::*;
use std::fs::File;

pub fn run(input: &str) {
    // constant definitions
    static TREE_SQUARE: u8 = '#' as u8;

    // todo: error handling for missing file
    let mut file = File::open(input).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    // we need to count the trees that we hit on our way down the hill
    let mut tree_count: u32 = 0;

    // build the 2d-vector representation of the map
    let mut map: Vec<Vec<u8>> = Vec::new();
    for line in contents.lines() {
        map.push(String::from(line).into_bytes());
    }

    // now we have a vec where the first dimension represents y axis,
    //   and the second dimension represents x axis
    // since we're always going down by 1 line, we can just iterate through
    //   the first dimension of the vec
    // we're always going right by 3 units, so keep track of that counter too
    let mut cur_x_pos: u32 = 0;
    for line in map.iter() {
        // the logical x-axis repeats forever... so mod cur_x_pos by the length
        //   of actual x-axis given in the input file
        if let Some(position) = line.get((cur_x_pos as usize) % line.len()) {
            if *position == TREE_SQUARE {
                tree_count += 1;
            }
            cur_x_pos += 3;
        }
    }

    println!("Tree count: {}", tree_count);
}