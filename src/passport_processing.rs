extern crate regex;

use regex::Regex;
use std::io::prelude::*;
use std::fs::File;
use std::iter::Iterator;

// configure what a blank line looks like depending on OS
#[cfg(windows)]
const BLANK_LINE: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
const BLANK_LINE: &'static str = "\n\n";

#[derive(PartialEq)]
enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
    Invalid,
}

enum HeightUnit {
    Centimeters,
    Inches,
    Invalid,
}

struct Passport {
    byr: u32,        // birth year
    iyr: u32,        // issue year
    eyr: u32,        // expiration year
    hgt: (HeightUnit, u32),  // height (u32 is the value)
    hcl: String,     // hair color
    ecl: EyeColor,   // eye color
    pid: String,     // passport ID
    cid: u32,        // country ID. If set to 0, the "country" is North Pole.
}

pub fn run(input: &str) {
    // todo: error handling for missing file
    let mut file = File::open(input).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut passports: Vec<Passport> = Vec::new();
    // construct passports from lines
    // each passport is separated by a blank line,
    //   so split lines along newline pairs
    for input_passport in contents.split(BLANK_LINE) {
        // start with an empty passport
        // unfortunately, enums don't support the Default trait, so we can't
        //   just use Default::default().
        let mut passport = Passport {
            byr: 0,
            iyr: 0,
            eyr: 0,
            hgt: (HeightUnit::Invalid, 0),
            hcl: String::new(),
            ecl: EyeColor::Invalid,
            pid: String::new(),
            cid: 0,
        };

        for field in input_passport.split_whitespace() {
            // for each field, the key and value are separated by a colon
            // i recognize that i really should be serializing this into json,
            //   but i'm not good enough at rust for that yet.
            let key_val: Vec<&str> = field.split(':').collect();
            // key_val.get(0) is the key
            if let Some(key) = key_val.get(0) {
                // key_val.get(1) is the value
                if let Some(val) = key_val.get(1) {
                    match *key {
                        "byr" => passport.byr = val.parse::<u32>().unwrap_or_default(),
                        "iyr" => passport.iyr = val.parse::<u32>().unwrap_or_default(),
                        "eyr" => passport.eyr = val.parse::<u32>().unwrap_or_default(),
                        "hgt" => {
                            if val.ends_with("cm") {
                                // get the value before the "cm" by slicing the
                                //   string at the "cm"
                                if let Some(unit_suffix_ix) = val.find("cm") {
                                    passport.hgt = (
                                        HeightUnit::Centimeters,
                                        // get the value before the "cm"
                                        val[0..unit_suffix_ix].parse::<u32>().unwrap_or_default(),
                                    );    
                                }
                            } else if val.ends_with("in") {
                                // get the value before the "in" by slicing the
                                //   string at the "in"
                                if let Some(unit_suffix_ix) = val.find("in") {
                                    passport.hgt = (
                                        HeightUnit::Inches,
                                        // get the value before the "cm"
                                        val[0..unit_suffix_ix].parse::<u32>().unwrap_or_default(),
                                    );    
                                }
                            } else {
                                // invalid height string
                                passport.hgt = (HeightUnit::Invalid, 0);
                            }
                        },
                        "ecl" => {
                            passport.ecl = match *val {
                                "amb" => EyeColor::Amber,
                                "blu" => EyeColor::Blue,
                                "brn" => EyeColor::Brown,
                                "gry" => EyeColor::Gray,
                                "grn" => EyeColor::Green,
                                "hzl" => EyeColor::Hazel,
                                "oth" => EyeColor::Other,
                                _     => EyeColor::Invalid,
                            }
                        }
                        "hcl" => passport.hcl = val.to_string(),
                        "pid" => passport.pid = val.to_string(),
                        "cid" => passport.cid = val.parse::<u32>().unwrap_or_default(),
                        _     => println!("Error! Found unknown key {}", key),
                    }
                } else {
                    println!("Error; no value for field {}", field);
                }
            } else {
                println!("Error; no key for field {}", field);
            }
        }

        passports.push(passport);
    }

    // Now we've built all the passports. Validate them and count the valid ones
    let mut valid_passports = 0;
    // this regex matches hex values starting with a pound sign and
    //   followed by 6 hex digits. Used to validate hair color.
    let hex_color_regex = Regex::new(r"#([a-f0-9]){6}").unwrap();

    for passport in passports.iter() {
        // make sure that all fields match expected values.
        // cid is the exception; cid can be missing.
        // this is a kind of elementary implementation... would be better to
        //   implement an Iterator for Passport, and some kind of rules engine
        //   or something. but this is just a silly daily programming challenge.

        if !((1920..=2002).contains(&passport.byr)) { continue; }
        if !((2010..=2020).contains(&passport.iyr)) { continue; }
        if !((2020..=2030).contains(&passport.eyr)) { continue; }

        if passport.pid.len() != 9 {
            continue;
        }

        match passport.hgt.0 {
            HeightUnit::Centimeters => {
                if !((150..=193).contains(&passport.hgt.1)) { continue; }
            },
            HeightUnit::Inches => {
                if !((59..=76).contains(&passport.hgt.1)) { continue; }
            },
            HeightUnit::Invalid => { continue; }
        }

        if !(hex_color_regex.is_match(passport.hcl.as_str())) { continue; }
        if passport.ecl == EyeColor::Invalid { continue; }

        // if we made it down here, we have either a valid passport,
        //   or a valid North Pole credential
        valid_passports += 1;
    }

    println!("Valid passports: {}", valid_passports);
}