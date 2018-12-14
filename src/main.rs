extern crate exitfailure;
extern crate ansi_term;

use exitfailure::ExitFailure;
use ansi_term::Colour::{Red, Green, Yellow};
use std::io;

fn main() -> Result<(), ExitFailure> {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                let input_str = input.as_str();
                let mut iter = input_str.split_whitespace();
                if let Some(ll) = iter.clone().position(|x| x == "W" || x == "D" || x == "I" || x == "E") {
                    match iter.nth(ll) {
                        Some("W") => println!("{}", Yellow.paint(input_str)),
                        Some("I") => println!("{}", Green.paint(input_str)),
                        Some("E") => println!("{}", Red.paint(input_str)),
                        Some("D") => println!("{}", input_str),
                        Some(_) => println!("{}", input_str),
                        None => {},
                    }
                }
            },
            Err(err) => {
                eprintln!("IO error: {}", err);
                break;
            }
        }
        input = "".to_string();
    }
    Ok(())
}
