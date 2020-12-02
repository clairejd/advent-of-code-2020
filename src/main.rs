use std::io;
// for report_repair (todo: remove)
use std::fs;

fn main() {
    println!("Advent of Code 2020 runner");
    println!("Enter the number of the program you'd like to run.");
    
    let mut program_was_run = false;

    while !(program_was_run) {        
        // get program number as a string from user
        let mut prog_number = String::new();
        io::stdin()
            .read_line(&mut prog_number)
            .expect("Failed to read line");

        let prog_number: u32 = match prog_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number.");
                continue
            },
        };
        
        // now we know we have a valid program number, run the program
        // if the program does not exist, run_program returns false
        let program_was_run = match run_program(prog_number);
    }
}

/******************************************************************/
/* Runs a program based on the program number.                    */
/* Returns true if program was run, returns false if no such      */
/*   program exists.                                              */
/******************************************************************/
fn run_program(prog_number: u32) -> i32 {
    match prog_number {
        1 => report_repair();
        _ => println!("Not a valid program number");
    }
}

/******************************************************/
/* TODO: split into its own file                      */
/******************************************************/
fn report_repair() {
    let mut file = std::fs::File::open("inputs/1.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    let mut numbers: Vec<i32> = Vec::new();
    for s in contents.lines() {
        numbers.push(s.parse::<i32>().unwrap());
    }

    // now we have a vector containing each number in the file
    
}