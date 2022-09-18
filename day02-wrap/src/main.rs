use std::fs;
use day02_wrap::*;


fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut input_vec: Vec<&str> = input.lines().collect();

    let mut element = paper::get_dimensions(input_vec.pop());
    let mut total_surface: usize = 0;
    let mut total_ribbon: usize = 0;

    while let Some(dimensions) = element {
        total_surface += paper::get_surface(&dimensions);
        total_ribbon += ribbon::get_ribbon(&dimensions);
        element = paper::get_dimensions(input_vec.pop());
    }

    println!("You should order {} square feet of wrapping paper. 🎁", total_surface);
    println!("You should also order {} feet of ribbon. 🎀", total_ribbon);
}
