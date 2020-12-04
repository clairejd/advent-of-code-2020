use std::io::prelude::*;
use std::fs::File;

struct Slope {
    down: usize,
    right: usize,
}

pub fn run(input: &str) {
    // constant definitions
    static TREE_SQUARE: u8 = '#' as u8;
    let slopes: [Slope; 5] = [
        Slope { down: 1, right: 1 },
        Slope { down: 1, right: 3 },
        Slope { down: 1, right: 5 },
        Slope { down: 1, right: 7 },
        Slope { down: 2, right: 1 },
    ];

    // todo: error handling for missing file
    let mut file = File::open(input).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // create a vector of numbers which we will eventually multiply together
    //    to get our final answer
    let mut nums_to_mult: Vec<u32> = Vec::new();

    // build the 2d-vector representation of the map
    let mut map: Vec<Vec<u8>> = Vec::new();
    for line in contents.lines() {
        map.push(String::from(line).into_bytes());
    }

    for slope in slopes.iter() {
        // we need to count the trees that we hit on our way down the hill
        let mut tree_count: u32 = 0;

        // now we have a vec where the first dimension represents y axis,
        //   and the second dimension represents x axis
        // keep track of the current X position
        let mut cur_x_pos: usize = 0;
        // use step_by to skip over rows if necessary
        for line in map.iter().step_by(slope.down) {
            // the logical x-axis repeats forever... so mod cur_x_pos by the length
            //   of actual x-axis given in the input file
            if let Some(position) = line.get((cur_x_pos as usize) % line.len()) {
                if *position == TREE_SQUARE {
                    tree_count += 1;
                }
                cur_x_pos += slope.right;
            }
        }
        nums_to_mult.push(tree_count);
    }

    // calculate product of tree counts
    let mut product: u32 = 1;
    for number in nums_to_mult.iter() {
        product *= *number;
    }

    println!("Product of tree counts: {}", product);
}