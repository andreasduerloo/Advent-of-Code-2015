use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut input_vec: Vec<&str> = input.lines().collect();
}
