use std::fs;
use std::collections:HashMap;
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
        println!("⭐ Deer {} ran {} km. 🦌", deer.name, run(&deer, 2503));
        // Get a vector of distances and return the highest
    }
    // Note: iterator has consumed the vector.
    
    // Make a HashMap of deer and their score
    let mut deer_map: HashMap<>;

    // Add one point to the deer in the lead after every second
    for i in 0..2503 {
        //
    }
}
