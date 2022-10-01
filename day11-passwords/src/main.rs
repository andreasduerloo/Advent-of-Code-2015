use std::env;
use day11_passwords::*;

fn main() {
    let mut arguments = env::args();

    match arguments.len() {
        0 | 1 => {
            println!("Missing argument - starting password");
            return
        }
        2 => {
            //
        }
        _ => {
            println!("Too many arguments.");
            return
        }
    }
}
