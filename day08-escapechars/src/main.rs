use std::fs;
use day08_escapechars::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut input_vec: Vec<&str> = input.lines().collect();

    let mut code_chars: usize = 0;
    let mut mem_chars: usize = 0;
    let mut expanded_chars: usize = 0;

    while let Some(content) = input_vec.pop() {
        characters::count_chars(content, &mut code_chars, &mut mem_chars, &mut expanded_chars);
    }

    println!("First star: {}, second star {}", code_chars - mem_chars, expanded_chars - code_chars);
}
