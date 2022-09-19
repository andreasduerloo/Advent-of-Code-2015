use std::fs;
use day06_lights::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let input_vec: Vec<&str> = input.lines().collect();

    //let mut grid: [[bool; 1000]; 1000] = [[false; 1000]; 1000]; STACK OVERFLOW
    let mut grid: Vec<Vec<(bool, usize)>> = vec![vec![(false, 0); 1000]; 1000]; // To the heap it is

    for instruction in input_vec {
        if let Some(parsed_instruction) = input::parse_line(instruction) {
            lights::execute_instruction(&mut grid, parsed_instruction);
        }
    }
    
    let mut on_count: usize = 0;
    let mut brightness: usize = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if grid[i][j].0 {
                on_count += 1;
            }
            brightness += grid[i][j].1
        }
    }

    println!("⭐ Lights ON after the instructions: {} 💡", on_count);
    println!("🌟 Total brightness after the instructions: {} 💡", brightness);
}
