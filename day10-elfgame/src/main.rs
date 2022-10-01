use std::env;
use day10_elfgame::*;

// input = 1321131112

fn main() {
    let mut arguments = env::args();

    match arguments.len() {
        0 | 1 | 2 => {
            println!("Too few arguments: puzzle input and number of iterations required.");
            return
        }
        3 => {
            let input = arguments.nth(1);
            let count = arguments.next();

            if let Some(text) = input {
                let char_vec: Vec<char> = text.chars().collect();
                if let Some(num_text)= count {
                    if let Ok(number) = usize::from_str_radix(&num_text, 10) {
                        let iterations: usize = number;
                        println!("After {} iterations the result is {} characters long.", iterations, play_game(char_vec, iterations));
                    }
                    else {
                        println!("Error: the second argument was not a valid (positive) integer.");
                        return
                    }
                }
                else {
                    println!("We're here, somehow");
                }
            }            
        }
        _ => {
            println!("Too many arguments.");
            return
        }
    }
}
