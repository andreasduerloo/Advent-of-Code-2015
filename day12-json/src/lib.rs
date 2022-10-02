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
        println!("Tried to turn something into an isize and it didn't work. Input was {}", out_string);
        0
    }
}