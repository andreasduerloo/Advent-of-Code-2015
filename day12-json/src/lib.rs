fn is_digit_or_minus(input_char: &char, valid_chars: &[char; 11]) -> bool {
    if valid_chars.contains(input_char) {
        true
    }
    else {
        false
    }
}

fn vec_to_isize(input_chars: Vec<char>) -> isize {
    let out_string: String = input_chars.into_iter().collect();
    if let Ok(out_num) = isize::from_str_radix(&out_string, 10) {
        out_num
    }
    else {
        panic!("Tried to turn something into an isize and it didn't work. Input was {}", out_string);
    }
}

fn is_inside_object(input_char: &char) -> Option<bool> {
    match input_char {
        '{' => {
            Some(true)
        },
        '}' => {
            Some(false)
        },
        _ => {
            None
        }
    }
}

pub fn run(characters: Vec<char>, valid_chars: &[char; 11]) -> isize {
    let mut position: usize = 0;
    let mut running_count: isize = 0;

    while position < characters.len() {
        if is_digit_or_minus(&characters[position], valid_chars) {
            let mut local_count: usize = 1;
            let mut digit_vec: Vec<char> = Vec::from([characters[position]]);

            while is_digit_or_minus(&characters[position + local_count], valid_chars) {
                digit_vec.push(characters[position + local_count]);
                local_count += 1;
            }
            running_count += vec_to_isize(digit_vec);
            position += local_count;
        }
        else {
            position += 1;
        }
    }
    running_count
}

pub fn run_without_red(characters: Vec<char>, valid_chars: &[char; 11]) -> isize {
    let mut object_flag: bool = false; // If objects can be nested we need to count brackets or push/pop them.
    let mut position: usize = 0;
    let mut running_count: isize = 0;
    let mut count_to_close: usize = 0;

    while position < characters.len() {
        println!("Current position: {}, char: {}", position, characters[position]);
        if let Some(toggle) = is_inside_object(&characters[position]) {
            object_flag = toggle;
        }

        if object_flag {
            // We're inside an object, scan for 'r','e','d' until closing brackets.
            let mut red_flag: bool = false;
            for local_position in position..characters.len() - 2 {
                if characters[local_position] == 'r' && characters[local_position + 1] == 'e' && characters[local_position + 2] == 'd' {
                    red_flag = true;
                }
                else {
                    if characters[local_position] == '}' {
                        count_to_close = local_position;
                        break
                    }
                }
            }
            if red_flag {
                position = count_to_close;
                object_flag = false;
            }
            else {
                position += 1;
            }
        }
        else {
            // We're not currently inside an object, keep going
            if is_digit_or_minus(&characters[position], valid_chars) {
                let mut local_count: usize = 1;
                let mut digit_vec: Vec<char> = Vec::from([characters[position]]);
    
                while is_digit_or_minus(&characters[position + local_count], valid_chars) {
                    digit_vec.push(characters[position + local_count]);
                    local_count += 1;
                }
                running_count += vec_to_isize(digit_vec);
                position += local_count;
            }
            else {
                position += 1;
            }
        }
    }
    running_count
}