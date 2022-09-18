use md5;
use std::env;

fn main() {
    let mut arguments = env::args();

    match arguments.len() {
        0 | 1 => {
            println!("Missing argument: secret key. 🔑");
            return;
        }
        2 => {
            if let Some(content) = arguments.nth(1) {
                let key = content;

                let mut first_found: bool = false;
                let mut second_found: bool = false;
                let mut number: usize = 1;

                while !first_found || !second_found {
                    let concat = format!("{}{}", key, number);

                    let hash = md5::compute(&concat);
                    let hash_hex = format!("{:x}", hash);
                    if hash_hex[0..5] == String::from("00000") && !first_found {
                        first_found = true;
                        println!("Hash found for number {} 🔓", number);
                    }
                    if hash_hex[0..6] == String::from("000000") && !second_found {
                        second_found = true;
                        println!("Hash found for number {} 🔓", number);
                    }
                    number += 1;
                }  
            }
        }
        _ => {
            println!("Too many arguments.");
            return;
        }
    }
}
