use std::fs;
use day14_deer::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let line_vec: Vec<&str> = input.lines().collect();

    let mut deer_vec: Vec<Deer> = Vec::new();

    for line in line_vec {
        let split: Vec<&str> = line.split_whitespace().collect();
        let name: &str = split[0];
        let speed: usize = usize::from_str_radix(split[3], 10).unwrap();
        let duration: usize = usize::from_str_radix(split[6], 10).unwrap();
        let rest_time: usize = usize::from_str_radix(split[13], 10).unwrap();
        let new_deer: Deer = Deer::new(name, speed, duration, rest_time);

        deer_vec.push(new_deer);
    }

    for deer in deer_vec {
        println!("Deer {} has a speed of {}", deer.name, deer.speed);
    }
}
