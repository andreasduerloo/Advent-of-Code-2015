use std::env;
use day11_passwords::*;
use std::collections::HashMap;

// Starting password: hxbxwxba

fn main() {
    let letters: [char; 26] = [ // For number -> letter conversion
        'a', 'b', 'c', 'd',
        'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l',
        'm', 'n', 'o', 'p',
        'q', 'r', 's', 't',
        'u', 'v', 'w', 'x',
        'y', 'z'
    ];

    let mut index: HashMap<char, usize> = HashMap::new(); // For letter -> number conversion

    for i in 0..26 {
        index.insert(letters[i], i);
    }

    let mut arguments = env::args();

    match arguments.len() {
        0 | 1 => {
            println!("Missing argument - starting password");
            return
        }
        2 => {
            let mut start_password = build_array(&arguments.nth(1).unwrap());

            while !check(&start_password, &index) {
                start_password = increment(start_password, &letters, &index);
            }
        }
        _ => {
            println!("Too many arguments.");
            return
        }
    }
}
