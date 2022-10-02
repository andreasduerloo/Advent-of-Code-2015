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