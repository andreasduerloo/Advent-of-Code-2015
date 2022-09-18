use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let char_vec: Vec<&str> = input.split("").collect();

    let mut floor: isize = 0;

    for instruction in char_vec {
        match instruction {
            "(" => {
                floor += 1;
            },
            ")" => {
                floor -= 1;
            },
            _ => {
                println!("Santa ends up on floor number {}", floor);
            }
        }
    }
}
