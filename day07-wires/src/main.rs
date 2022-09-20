use std::fs;
use day07_wires::*;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let input_vec: Vec<&str> = input.lines().collect();

    let mut wires: HashMap<&str,u16> = HashMap::new();

    // Step one, get the lines of format number -> wire. Put those values into a hashmap
    let mut new_vec: Vec<Vec<&str>> = filter::first_pass(input_vec, &mut wires);

    while new_vec.len() != 0 {
        new_vec = filter::pass(new_vec, &mut wires);
        // println!("There are {} instructions left after this pass", new_vec.len());
    }

    println!("The value of wire 'a' is {} ⚡", wires.get("a").unwrap());
}
