use std::fs;
// use std::collections:HashMap;
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

    // First star
    let mut dist_vec: Vec<usize> = Vec::new();
    for i in 0..deer_vec.len() {
        println!("⭐ Deer {} ran {} km. 🦌", deer_vec[i].name, run(&deer_vec[i], 2503));
        dist_vec.push(run(&deer_vec[i], 2503));
    }

    println!("--- ⭐ First star ⭐ ---");
    println!("The longest distance is {} km.", longest(&dist_vec));
    
    // Second star
    for i in 1..2504 {
        let mut dist_vec: Vec<usize> = Vec::new();
        for j in 0..deer_vec.len() {
            let ran: usize = run(&deer_vec[j], i);
            deer_vec[j].current_distance = ran;
            dist_vec.push(ran);
        }

        for k in 0..deer_vec.len() {
            if deer_vec[k].current_distance == longest(&dist_vec) {
                deer_vec[k].score += 1;
            }
        }
    }

    println!("--- ⭐ Second star ⭐ ---");
    println!("The highest score is {}.", highest_score(&deer_vec));
}
