use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let char_vec: Vec<&str> = input.split("").collect();

    let mut houses: HashMap<[isize; 2],usize> = HashMap::new();
    let mut current_location: [isize; 2] = [0, 0];

    // Santa starts at the origin
    houses.insert([0,0], 1);

    for symbol in char_vec {
        match symbol {
            "^" => {
                current_location[1] = current_location[1] + 1;
                let presents = houses.entry(current_location).or_insert(1);
                *presents += 1;
            },
            "v" => {
                current_location[1] = current_location[1] - 1;
                let presents = houses.entry(current_location).or_insert(1);
                *presents += 1;
            },
            ">" => {
                current_location[0] = current_location[0] + 1;
                let presents = houses.entry(current_location).or_insert(1);
                *presents += 1;
            },
            "<" => {
                current_location[0] = current_location[0] - 1;
                let presents = houses.entry(current_location).or_insert(1);
                *presents += 1;
            },
            _ => {
                continue
            }
        }
    }
    println!("Houses with at least one gift: {}", houses.len());
}
