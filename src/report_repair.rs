use std::io::prelude::*;
use std::fs::File;

pub fn run(input: &str) {
    // todo: error handling for missing file
    let mut file = File::open(input).unwrap();
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