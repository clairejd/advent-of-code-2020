use std::io::prelude::*;
use std::fs::File;
use std::process;

// constant definitions
// number of times we can partition rows and columns
const ROW_PARTITIONS: usize = 7;
const COL_PARTITIONS: usize = 3;

#[derive(Clone, Copy, Debug)]
enum RowPartition {
    Front,
    Back,
}

#[derive(Clone, Copy, Debug)]
enum ColumnPartition {
    Left,
    Right,
}

#[derive(Debug)]
struct BoardingPass {
    row_directions: [Option<RowPartition>; ROW_PARTITIONS],
    column_directions: [Option<ColumnPartition>; COL_PARTITIONS],
}

// Construct and return a vec of BoardingPass structs from the input string
fn build_boarding_passes(input: &str) -> Vec<BoardingPass> {
    let mut passes: Vec<BoardingPass> = Vec::new();
    
    for input_pass in input.lines() {
        // initialize BoardingPass as empty
        let mut pass = BoardingPass { 
            row_directions: [None; ROW_PARTITIONS],
            column_directions: [None; COL_PARTITIONS],
        };

        // an input pass is represented by ROW_PARTITIONS + COL_PARTITIONS chars
        // the first ROW_PARTITIONS chars are "F" or "B" for "front"/"back"
        for (ix, current_char) in input_pass.chars().enumerate() {
            // if index is less than ROW_PARTITIONS, this should be a
            //   row instruction
            if ix < ROW_PARTITIONS {
                pass.row_directions[ix] = match current_char {
                    'F' => Some(RowPartition::Front),
                    'B' => Some(RowPartition::Back),
                    _   => {
                        println!("Error! Unexpected row char {}", current_char);
                        process::exit(1);
                    }
                }
            } else {
                // index is >= ROW_PARTITIONS, so it must be a column instr
                // the index of column directions should start at 0, not at
                //   ROW_PARTITIONS like the input string does
                pass.column_directions[ix - ROW_PARTITIONS] = match current_char {
                    'L' => Some(ColumnPartition::Left),
                    'R' => Some(ColumnPartition::Right),
                    _   => {
                        println!("Error! Unexpected col char {}", current_char);
                        process::exit(1);
                    }
                }
            }
        }
        
        passes.push(pass);
    }

    return passes;
}

pub fn run(input: &str) {
    // total number of rows and columns on the plane
    let TOTAL_ROWS: u32 = 2_u32.pow(ROW_PARTITIONS as u32);
    let TOTAL_COLS: u32 = 2_u32.pow(COL_PARTITIONS as u32);

    // todo: error handling for missing file
    let mut file = File::open(input).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let boarding_passes = build_boarding_passes(&contents);
}