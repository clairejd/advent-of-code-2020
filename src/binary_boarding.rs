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
        let total_rows: u32 = 2_u32.pow(ROW_PARTITIONS as u32) - 1;
        // use binary partition search to get the actual row number
        return self.get_row_recurse(0, 0, total_rows);
    }

    // Get the column number based on this pass's column directions
    pub fn get_col(&self) -> u32 {
        // total number of columns on the plane
        let total_cols: u32 = 2_u32.pow(COL_PARTITIONS as u32) - 1;
        return self.get_col_recurse(0, 0, total_cols);
    }


    // use binary partition search to get the actual row number
    // dir_ix:  the array index of self.row_directions to query to determine
    //          whether to go forward or backward
    // min_row: the minimum row of the current partition
    // max_row: the maximum row of the current partition
    fn get_row_recurse(&self, dir_ix: usize, min_row: u32, max_row: u32) -> u32 {
        match max_row - min_row {
            // if there is only a difference of 1 between max and min, we've
            //   reached the end of our recursion. use the last row direction to
            //   return the appropriate row number
            1 => {
                if let Some(dir) = self.row_directions[dir_ix] {
                    match dir {
                        RowPartition::Front => return min_row,
                        RowPartition::Back  => return max_row,
                    }
                } else {
                    println!("No direction for index {}", dir_ix);
                    return 0;
                }
            },

            // else, use the direction at this index to determine whether to go
            //  forward or backward
            _ => {
                if let Some(dir) = self.row_directions[dir_ix] {
                    // depending on the direction that we have to go in, we will
                    //   be setting a new min/max row number to send to the next
                    //   recursive call to this function.
                    let mut new_min_row = min_row;
                    let mut new_max_row = max_row;
                    // we also need to advance to the next direction index.
                    let new_dir_ix = dir_ix + 1;
                    
                    // calculate the midpoint between the two current rows.
                    // if we go to the front, the midpoint is our new max row.
                    // if we go to the back, the midpoint is our new min row.
                    let midpoint = (min_row + max_row) / 2;
                    match dir {
                        RowPartition::Front => new_max_row = midpoint,
                        RowPartition::Back  => new_min_row = midpoint + 1,
                    }

                    return self.get_row_recurse(new_dir_ix, new_min_row, new_max_row);
                } else {
                    println!("Error: got no direction for index {}!", dir_ix);
                    return 0;
                }
            }
        }
    }

    fn get_col_recurse(&self, dir_ix: usize, min_col: u32, max_col: u32) -> u32 {
        match max_col - min_col {
            // if there is only a difference of 1 between max and min, we've
            //   reached the end of our recursion. use the last col direction to
            //   return the appropriate col number
            1 => {
                if let Some(dir) = self.column_directions[dir_ix] {
                    match dir {
                        ColumnPartition::Left  => return min_col,
                        ColumnPartition::Right => return max_col,
                    }
                } else {
                    println!("No direction for index {}", dir_ix);
                    return 0;
                }
            },

            // else, use the direction at this index to determine whether to go
            //  forward or backward
            _ => {
                if let Some(dir) = self.column_directions[dir_ix] {
                    // depending on the direction that we have to go in, we will
                    //   be setting a new min/max col number to send to the next
                    //   recursive call to this function.
                    let mut new_min_col = min_col;
                    let mut new_max_col = max_col;
                    // we also need to advance to the next direction index.
                    let new_dir_ix = dir_ix + 1;
                    
                    // calculate the midpoint between the two current cols.
                    // if we go to the front, the midpoint is our new max col.
                    // if we go to the back, the midpoint is our new min col.
                    let midpoint = (min_col + max_col) / 2;
                    match dir {
                        ColumnPartition::Left  => new_max_col = midpoint,
                        ColumnPartition::Right => new_min_col = midpoint + 1,
                    }

                    return self.get_col_recurse(new_dir_ix, new_min_col, new_max_col);
                } else {
                    println!("Error: got no direction for index {}!", dir_ix);
                    return 0;
                }
            }
        }
    }
}

struct Seat {
    row: u32,
    col: u32,
}

impl Seat {
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
    for pass in passes.iter() {
        seats.push(Seat {
            row: pass.get_row(),
            col: pass.get_col(),
        });
    }

    return seats;
}

pub fn run(input: &str) {
    // todo: error handling for missing file
    let mut file = File::open(input).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let boarding_passes = build_boarding_passes(&contents);
    let seats = build_seats(&boarding_passes);
    if let Some(max_seat_id) = seats.iter().map(|seat| seat.get_seat_id()).max() {
        println!("Max seat ID: {}", max_seat_id);
    }
}