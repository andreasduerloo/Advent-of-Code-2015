use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let char_vec: Vec<&str> = input.split("").collect();

    let mut floor: isize = 0;
    let mut character: usize = 0; // There appears to be some leading character
    let mut basement: bool = false;

    for instruction in char_vec {
        match instruction {
            "(" => {
                floor += 1;
                character += 1;
            },
            ")" => {
                floor -= 1;
                character += 1;
            },
            _ => {
                println!("Santa ends up on floor number {} 🎅", floor);
            }
        }
        if floor < 0 && !basement {
            println!("We're in the basement for the first time at character {}", character);
            basement = true;
        }
    }
}
