use std::io::prelude::*;
use std::fs::File;

// configure what a blank line looks like depending on OS
#[cfg(windows)]
const BLANK_LINE: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const BLANK_LINE: &'static str = "\n\n";

const NUMBER_OF_QUESTIONS: usize = 26;
const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

struct AnswerGroup {
    // an AnswerGroup is a vector of bool arrays. Each element in the vector
    //   represents one person's answers. Each element in the array of answers
    //   represents either the yes (true) or no (false) answer.
    personal_answers: Vec<[bool; NUMBER_OF_QUESTIONS]>,
}

impl AnswerGroup {
    // get_combined_affirmative_answers returns the total number of questions
    //  that everyone in this group answered yes to.
    pub fn get_combined_affirmative_answers(&self) -> u32 {
        let mut combined_answers: [bool; NUMBER_OF_QUESTIONS] = [true; NUMBER_OF_QUESTIONS];
        for person in self.personal_answers.iter() {
            for (ix, answer) in person.iter().enumerate() {
                if *answer == false {
                    combined_answers[ix] = false;
                }
            }
        }

        let mut combined_yes_count = 0;
        for ans in combined_answers.iter() {
            if *ans == true {
                combined_yes_count += 1;
            }
        }
        
        return combined_yes_count;
    }
}

// Construct and return a vec of BoardingPass structs from the input string
fn build_answer_groups(input: &str) -> Vec<AnswerGroup> {
    let mut groups: Vec<AnswerGroup> = Vec::new();
    
    for input_group in input.split(BLANK_LINE) {
        // initialize BoardingPass as empty
        let mut group = AnswerGroup { 
            personal_answers: Vec::new(),
        };

        // each person in the group is represented by a new line
        for person in input_group.lines() {
            let mut input_person: [bool; NUMBER_OF_QUESTIONS] = [ false; NUMBER_OF_QUESTIONS ];
            // each character in the line represents a yes answer
            for letter in person.chars() {
                if let Some(answer_ix) = ALPHABET.iter().position(|&c| c == letter) {
                    input_person[answer_ix] = true;
                }
            }
            group.personal_answers.push(input_person);
        }
        groups.push(group);
    }

    return groups;
}

pub fn run(input: &str) {
    // todo: error handling for missing file
    let mut file = File::open(input).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let answer_groups = build_answer_groups(&contents);
    let group_answer_totals: Vec<u32> = answer_groups
        .iter()
        .map(|grp| grp.get_combined_affirmative_answers())
        .collect();
    let sum: u32 = group_answer_totals.iter().sum();
    println!("Sum of yes answers: {}", sum);
}