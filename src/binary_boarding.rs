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

impl BoardingPass {
    // Get the row number based on this pass's row directions
    pub fn get_row(&self) -> u32 {
        // total number of rows on the plane
        let TOTAL_ROWS: u32 = 2_u32.pow(ROW_PARTITIONS as u32);
        // use binary partition search to get the actual row number
        get_row_recurse(self, 0, 0, TOTAL_ROWS);
    }

    // use binary partition search to get the actual row number
    // dir_ix:  the array index of self.row_directions to query to determine
    //          whether to go forward or backward
    // min_row: the minimum row of the current partition
    // max_row: the maximum row of the current partition
    fn get_row_recurse(&self, dir_ix: u32, min_row: u32, max_row: u32) -> u32 {
        match (max_row - min_row) {
            // if there is only a difference of 1 between max and min, we've
            //   reached the end of our recursion. use the last row direction to
            //   return the row number
            1 => ,
            // else, use the direction at this index to determine whgeterh
            _
        }
    }

    // Get the column number based on this pass's column directions
    pub fn get_col(&self) -> u32 {
        // total number of columns on the plane
        let TOTAL_COLS: u32 = 2_u32.pow(COL_PARTITIONS as u32);
        get_col_recurse(self, 0, TOTAL_COLS);
    }

    // use binary partition search to get the actual column number
    fn get_col_recurse(&self, max_row: u32) -> u32 {

    }
}

struct Seat {
    row: u32,
    col: u32,
}

impl Seat {
    fn new(row: u32, col: u32) -> Seat {
        Seat { row: row, col: col }
    }

    fn get_seat_id(&self) -> u32 {
        (self.row * 8) + self.col
    }
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

// Construct and return a vec of Seat structs from the input boarding pass vec
fn build_seats(passes: &Vec<BoardingPass>) -> Vec<Seat> {
    let mut seats: Vec<Seat> = Vec::new();
    for pass in boarding_passes.iter() {
        seats.push(Seat { 
            row: 
        });
    }
}

pub fn run(input: &str) {
    // todo: error handling for missing file
    let mut file = File::open(input).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let boarding_passes = build_boarding_passes(&contents);
    let seats = build_seats(&boarding_passes);
}