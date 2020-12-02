use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    println!("Advent of Code 2020 runner");
    println!("Enter the number of the program you'd like to run.");
    
    // get program number as a string from user
    let mut prog_number = String::new();
    io::stdin()
        .read_line(&mut prog_number)
        .expect("Failed to read line");

    // todo: improve error checking
    let prog_number = prog_number
        .trim()
        .parse()
        .expect("not a number!");
        
    run_program(prog_number);
}

/******************************************************************/
/* Runs a program based on the program number.                    */
/* Returns true if program was run, returns false if no such      */
/*   program exists.                                              */
/******************************************************************/
fn run_program(prog_number: u32) {
    match prog_number {
        1 => report_repair(),
        _ => println!("Not a valid program number"),
    }
}

/******************************************************/
/* TODO: split into its own file                      */
/******************************************************/
fn report_repair() {
    // todo: error handling for missing file
    let mut file = File::open("inputs/1.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    let mut numbers: Vec<i32> = Vec::new();
    for s in contents.lines() {
        numbers.push(s.parse::<i32>().unwrap());
    }

    // sort the vector
    numbers.sort();
    
    // simple, stupid O(n^3) solution
    // maintain two iterators, start both at the beginning of the list
    // add each pair of numbers together until you find the answer
    for low_num in numbers.iter() {
        let mut done = false;
        for high_num in numbers.iter() {
            for mid_num in numbers.iter() {
                if low_num + mid_num + high_num == 2020 {
                    println!("found it! {}", low_num * mid_num * high_num);
                    done = true;
                    break;
                }
            }
            if done {
                break;
            }
        }
        if done {
            break;
        }
    }
}