use std::io::prelude::*;
use std::fs::File;

struct PasswordPolicy {
    first_ix: usize,
    second_ix: usize,
    letter: char,
}

struct InputLine<'a> {
    policy: PasswordPolicy,
    password: &'a str,
}

pub fn run(input: &str) {
    // todo: error handling for missing file
    let mut file = File::open(input).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // construct data structure from lines
    let mut input_lines: Vec<InputLine> = Vec::new();
    for line in contents.lines() {
        // split at the colon-and-space
        // this givs us the policy and password separated
        let line_sections: Vec<&str> = line.split(": ").collect();

        // parse the policy into parts separated by - and a space.
        // syntax: <min>-<max> <letter>
        let policy: Vec<&str> = line_sections[0].split(|c| c == '-' || c == ' ').collect();
        input_lines.push(InputLine {
           policy: PasswordPolicy {
               first_ix: policy[0].parse().unwrap(),
               second_ix: policy[1].parse().unwrap(),
               letter: policy[2].chars().nth(0).unwrap(),
           },
           password: line_sections[1],
        });
    }

    // count valid passwords
    let mut count = 0;
    for line in input_lines.iter() {
        let first_char = line.password
            .chars()
            .nth(line.policy.first_ix - 1)
            .unwrap();
        let second_char = line.password
            .chars()
            .nth(line.policy.second_ix - 1)
            .unwrap();
        if ((first_char == line.policy.letter) ||
                (second_char == line.policy.letter))
            &&
            (first_char != second_char) {
                count += 1;
        }
    }

    println!("{}", count);
}