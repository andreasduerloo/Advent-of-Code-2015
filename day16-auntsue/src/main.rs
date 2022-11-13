use std::fs;
use day16_auntsue::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let line_vec: Vec<&str> = input.lines().collect();

    let mut sue_vec: Vec<Sue> = Vec::new();
    let input_vec: Vec<&str> = Vec::new();

    for line in line_vec {
        let input_vec: Vec<&str> = line.split_whitespace().collect();
        let number: usize = usize::from_str_radix(input_vec[1][..-1], 10).unwrap();

        let temp_sue = Sue::new(number);

        for i in (3..9).step_by(2) {
            if input_vec[i + 1][-1] == "," {
                let number: usize = usize::from_str_radix(input_vec[i + 1][..-1], 10).unwrap();
            }
            else {
                let number: usize = usize::from_str_radix(input_vec[i + 1], 10).unwrap();
            }
            
            match input_vec[i] {
                "children" => {
                    temp_sue.children = Some(number);
                },
                "cats" => {
                    temp_sue.cats = Some(number);
                },
                "samoyeds" => {
                    temp_sue.samoyeds = Some(number);
                },
                "pomeranians" => {
                    temp_sue.pomeranians = Some(number);
                },
                "akitas" => {
                    temp_sue.akitas = Some(number);
                },
                "vizslas" => {
                    temp_sue.vizslas = Some(number);
                },
                "goldfish" => {
                    temp_sue.goldfish = Some(number);
                },
                "trees" => {
                    temp_sue.trees = Some(number);
                },
                "cars" => {
                    temp_sue.cars = Some(number);
                },
                "perfumes" => {
                    temp_sue.perfumes = Some(number);
                }
                _ => continue
            } 
        }

        sue_vec.push(temp_sue);
    }
}
