/******************************************************************************/
/* Advent of Code 2020 day 7:                                                 */
/*   -- Handheld Halting --                                                   */
/******************************************************************************/
/******************************************************************************/
/* Dependencies                                                               */
/******************************************************************************/
extern crate nom;

use nom::{
    alt,
    char,
    character::is_digit,
    map_res,
    recognize,
    tag,
    take_while,
    named
};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/******************************************************************************/
/* Constant definitions                                                       */
/******************************************************************************/

/******************************************************************************/
/* Structure/enum definitions                                                 */
/******************************************************************************/
#[derive(Clone, Copy, Debug)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    opcode: Operation,
    argument: usize,
    // if arg_positive is true, the argument is meant to be treated as a positive number.
    //   else, it should be considered negative.
    arg_positive: bool,
}

/******************************************************************************/
/* Parser definitions                                                         */
/******************************************************************************/
// recognize! returns the input string rather than the u8 slice returned by the parser
named!(parse_instr<&[u8], &str>,
    map_res!(
        recognize!(
            alt!( tag!( "acc" ) | tag!( "jmp" ) | tag!( "nop" ) )
        ),
        std::str::from_utf8
    )
);

named!(parse_number<&[u8], usize>,
    map_res!(
        recognize!(
            take_while!( is_digit ) 
        ),
        |bytes: &[u8]| std::str::from_utf8(bytes).unwrap().parse::<usize>()
    )
);

named!(parse_sign<&[u8], char>,  alt!( char!( '+' ) | char!( '-' ) ) );

named!(parse_single_space<&[u8], char>, char!( ' ' ) );

/******************************************************************************/
/* Subroutines                                                                */
/******************************************************************************/
// convert an input line to an instruction
fn line_to_inst(input: &str) -> Instruction {
    // parse one line.
    // first, get the operation
    let input = input.as_bytes();
    let (input, opcode) = parse_instr(input).unwrap();
    // next, skip whitespace
    let (input, _) = parse_single_space(input).unwrap();
    // get the sign...
    let (input, sign) = parse_sign(input).unwrap();
    // and finally the numerical argument
    let (_, num) = parse_number(input).unwrap();

    Instruction {
        opcode: match opcode {
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            "nop" => Operation::Nop,
            _     => panic!("Invalid opcode {}", opcode)
        },

        argument: num,
        
        arg_positive: match sign {
            '-' => false,
            '+' => true,
            _   => panic!("Invalid sign!")
        }
    }
}


fn parse_input(input: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    let file = File::open(input)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    while let Ok(line_length) = reader.read_line(&mut line) {
        match line_length {
            // if line_length is 0, we've reached end of file
            0 => break,
            _ => {
                instructions.push(line_to_inst(&line));
                line.clear();
            }
        }
    }

    Ok(instructions)
}

pub fn run_machine(input: Vec<Instruction>) -> Option<i32> {
    // pair each instruction with a bool representing if the instruction has been
    //   visited or not.
    let mut visited_instrs: Vec<(Instruction, bool)> = Vec::new();
    for instr in input.iter() {
        visited_instrs.push((*instr, false));
    }

    let mut accumulator: i32 = 0;
    let mut instruction_index = 0;
    
    // execute instructions
    while instruction_index < visited_instrs.len() {
        let mut instr = visited_instrs.get_mut(instruction_index).unwrap();
        match instr.1 {
            true => return None,
            false => {
                match instr.0.opcode {
                    Operation::Acc => {
                        if instr.0.arg_positive {
                            accumulator += instr.0.argument as i32;
                        } else {
                            accumulator -= instr.0.argument as i32;
                        }
                        instruction_index += 1;
                    },
                    Operation::Jmp => {
                        if instr.0.arg_positive {
                            instruction_index += instr.0.argument;
                        } else {
                            instruction_index -= instr.0.argument;
                        }
                    },
                    Operation::Nop => {
                        instruction_index += 1;
                    },
                }
                instr.1 = true;
            }
        }
    }

    // if we made it here, there was no loop in the machine;
    //   the machine halted successfully.
    // return the accumulator value
    return Some(accumulator);
}

/******************************************************************************/
/* Main routine                                                               */
/******************************************************************************/
pub fn run(input: &str) {
    // parse input file and build a vec of instructions
    let instructions: Vec<Instruction> = parse_input(input).unwrap();

    loop {
        // create a mutable copy of the instructions so we can modify one
        let mut modified_instructions = instructions.clone();
        for instr in modified_instructions {
            match instr.opcode {
                // if the operation is an Acc, continue... all Accs are correct instructions
                Operation::Acc => continue,
    
                // if it's a Jmp, convert it to a Nop and run the machine
                Operation::Jmp => {
                    instr.opcode = Operation::Nop;
                    let accumulator = run_machine(instructions);
                    match accumulator {
                        Some(result) => {
                            println!("Found result! {}", result);
                            break;
                        },
                        None => {
                            // turn instruction back into a Jmp
                            instr.opcode = Operation::Jmp;
                        }
                    }
                },
                
                // if it's a Nop, convert it to a Jmp and run the machine
                Operation::Nop => {
                    instr.opcode = Operation::Jmp;
                    let accumulator = run_machine(instructions);
                    match accumulator {
                        Some(result) => {
                            println!("Found result! {}", result);
                            break;
                        },
                        None => {
                            // turn instruction back into a Nop
                            instr.opcode = Operation::Nop;
                        }
                    }
                }
            }
        }
    }

}