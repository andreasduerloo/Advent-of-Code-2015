use std::fs;
use day12_json::*;

fn main() {
    /* We pass these values to a function that checks whether a given character is part of an integer.
    Because we will be calling the function a lot of times, I don't want to declare this array inside the function.
    */
    let valid_chars: [char; 11] = [
        '-', '0', '1', '2',
        '3', '4', '5', '6',
        '7', '8', '9'
    ];

    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let char_vec: Vec<char> = input.chars().collect();

    println!("First star - result = {}", run(char_vec, &valid_chars));

    // Go again for the second star
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let char_vec: Vec<char> = input.chars().collect();

    // Are there nested objects?
    
    // println!("Second star - result = {}", run_without_red(char_vec, &valid_chars));
}
