use std::fs;
use day06_lights::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut input_vec: Vec<&str> = input.lines().collect();

    // Initialize the grid, one million bytes = 1MB. Should be fine?
    //let mut grid: [[bool; 1000]; 1000] = [[false; 1000]; 1000]; STACK OVERFLOW
    let mut grid: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000];

    // Debug
    println!("{}", grid[153][854]);

    // Debug
    for instruction in input_vec {
        if let Some(parsed_instruction) = input::parse_line(instruction) {
            let mut cmd = String::new();
            match parsed_instruction.0 {
                input::Command::Toggle => {
                    cmd = String::from("Toggle");
                },
                input::Command::Off => {
                    cmd = String::from("Off");
                },
                input::Command::On => {
                    cmd = String::from("On");
                }
            }
            println!("Instruction: {}, points: {},{} and {},{}", cmd, parsed_instruction.1[0][0], parsed_instruction.1[0][1], parsed_instruction.1[1][0], parsed_instruction.1[1][1]);
        }
    }        
}

// 💡