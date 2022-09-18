use std::fs;
use day02_wrap::paper::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut input_vec: Vec<&str> = input.lines().collect();

    let mut element = get_dimensions(input_vec.pop());
    let mut total_surface: usize = 0;

    while let Some(dimensions) = element {
        total_surface += get_surface(&dimensions);
        element = get_dimensions(input_vec.pop());
    }

    println!("You should order {} square feet of wrapping paper. 🎁", total_surface);
}
