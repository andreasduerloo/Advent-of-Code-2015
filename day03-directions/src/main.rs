use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let char_vec: Vec<&str> = input.split("").collect();

    let mut houses: HashMap<[isize; 2],usize> = HashMap::new();
    let mut more_houses: HashMap<[isize; 2],usize> = HashMap::new();
    let mut current_location: [isize; 2] = [0, 0];
    let mut santa_location: [isize; 2] = [0, 0];
    let mut robo_location: [isize; 2] = [0, 0];
    let mut santas_turn: bool = true;

    // Santa starts at the origin
    houses.insert([0,0], 1);
    more_houses.insert([0,0], 2);

    for symbol in char_vec {
        match symbol {
            "^" => {
                current_location[1] = current_location[1] + 1;
                let presents = houses.entry(current_location).or_insert(1);
                *presents += 1;

                if santas_turn {
                    santa_location[1] = santa_location[1] + 1;

                    let presents = more_houses.entry(santa_location).or_insert(1);
                    *presents += 1;
                }
                else {
                    robo_location[1] = robo_location[1] + 1;

                    let presents = more_houses.entry(robo_location).or_insert(1);
                    *presents += 1; 
                }
                santas_turn = !santas_turn;
            },
            "v" => {
                current_location[1] = current_location[1] - 1;
                let presents = houses.entry(current_location).or_insert(1);
                *presents += 1;

                if santas_turn {
                    santa_location[1] = santa_location[1] - 1;

                    let presents = more_houses.entry(santa_location).or_insert(1);
                    *presents += 1;
                }
                else {
                    robo_location[1] = robo_location[1] - 1;

                    let presents = more_houses.entry(robo_location).or_insert(1);
                    *presents += 1; 
                }
                santas_turn = !santas_turn;
            },
            ">" => {
                current_location[0] = current_location[0] + 1;
                let presents = houses.entry(current_location).or_insert(1);
                *presents += 1;

                if santas_turn {
                    santa_location[0] = santa_location[0] + 1;

                    let presents = more_houses.entry(santa_location).or_insert(1);
                    *presents += 1;
                }
                else {
                    robo_location[0] = robo_location[0] + 1;

                    let presents = more_houses.entry(robo_location).or_insert(1);
                    *presents += 1; 
                }
                santas_turn = !santas_turn;
            },
            "<" => {
                current_location[0] = current_location[0] - 1;
                let presents = houses.entry(current_location).or_insert(1);
                *presents += 1;

                if santas_turn {
                    santa_location[0] = santa_location[0] - 1;

                    let presents = more_houses.entry(santa_location).or_insert(1);
                    *presents += 1;
                }
                else {
                    robo_location[0] = robo_location[0] - 1;

                    let presents = more_houses.entry(robo_location).or_insert(1);
                    *presents += 1; 
                }
                santas_turn = !santas_turn;
            },
            _ => {
                continue
            }
        }
    }
    println!("Year 1 - Houses with at least one gift: {}", houses.len());
    println!("Year 2 - Houses with at least one gift: {}", more_houses.len());
}
