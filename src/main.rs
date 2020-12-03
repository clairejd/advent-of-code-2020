#[macro_use]
extern crate clap;

use clap::App;

mod report_repair;
mod password_philosophy;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let input = matches.value_of("input").unwrap();
    
    match matches.value_of("program").unwrap() {
        "report" => report_repair::run(input),
        "password" => password_philosophy::run(input),
        _ => println!("Bad program number!"),
    }
}
