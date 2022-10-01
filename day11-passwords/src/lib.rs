use std::collections::HashMap;

// There is probably a better way
pub fn increment(mut password: [char; 8], letters: &[char; 26], indices: &HashMap<char, usize>) -> [char; 8] {
    match password[7] {
        'z' => {
            match password[6] {
                'z' => {
                    match password[5] {
                        'z' => {
                            match password[4] {
                                'z' => {
                                    match password[3] {
                                        'z' => {
                                            match password[2] {
                                                'z' => {
                                                    match password[1] {
                                                        'z' => {
                                                            match password[0] {
                                                                'z' => {
                                                                    password = ['a'; 8];
                                                                },
                                                                _ => {
                                                                    let index = indices.get(&password[0]).unwrap();
                                                                    password[0] = letters[*index + 1];
                                                                    password[1] = 'a';
                                                                    password[2] = 'a';
                                                                    password[3] = 'a';
                                                                    password[4] = 'a';
                                                                    password[5] = 'a';
                                                                    password[6] = 'a';
                                                                    password[7] = 'a';
                                                                }
                                                            }
                                                        },
                                                        _ => {
                                                            let index = indices.get(&password[1]).unwrap();
                                                            password[1] = letters[*index + 1];
                                                            password[2] = 'a';
                                                            password[3] = 'a';
                                                            password[4] = 'a';
                                                            password[5] = 'a';
                                                            password[6] = 'a';
                                                            password[7] = 'a';
                                                        }
                                                    }
                                                },
                                                _ => {
                                                    let index = indices.get(&password[2]).unwrap();
                                                    password[2] = letters[*index + 1];
                                                    password[3] = 'a';
                                                    password[4] = 'a';
                                                    password[5] = 'a';
                                                    password[6] = 'a';
                                                    password[7] = 'a';
                                                }
                                            }
                                        },
                                        _ => {
                                            let index = indices.get(&password[3]).unwrap();
                                            password[3] = letters[*index + 1];
                                            password[4] = 'a';
                                            password[5] = 'a';
                                            password[6] = 'a';
                                            password[7] = 'a';
                                        }
                                    }
                                },
                                _ => {
                                    let index = indices.get(&password[4]).unwrap();
                                    password[4] = letters[*index + 1];
                                    password[5] = 'a';
                                    password[6] = 'a';
                                    password[7] = 'a';
                                }
                            }
                        },
                        _ => {
                            let index = indices.get(&password[5]).unwrap();
                            password[5] = letters[*index + 1];
                            password[6] = 'a';
                            password[7] = 'a';
                        }
                    }
                },
                _ => {
                    let index = indices.get(&password[6]).unwrap();
                    password[6] = letters[*index + 1];
                    password[7] = 'a';
                }
            }
        },
        _ => {
            let index = indices.get(&password[7]).unwrap();
            password[7] = letters[*index + 1];
        }
    }
    password
}

pub fn check(password: &[char; 8], indices: &HashMap<char, usize>) -> bool {
    println!("Trying password: {}{}{}{}{}{}{}{}", password[0], password[1], password[2], password[3], password[4], password[5], password[6], password[7]);
    // Easiest checks first
    for i in 0..password.len() { // Any forbidden letters
        if password[i] == 'i' || password[i] == 'o' || password[i] == 'l' {
            return false;
        }
    }

    let mut straight: bool = false;

    for i in 0..password.len()-2 {
        if *indices.get(&password[i]).unwrap() < 24 {
            let start_index: usize = *indices.get(&password[i]).unwrap();
            let middle_index: usize = *indices.get(&password[i + 1]).unwrap();
            let end_index: usize = *indices.get(&password[i + 2]).unwrap(); // It goes wrong here when trying to get 'z'
    
            if start_index + 1 == middle_index && middle_index + 1 == end_index {
                straight = true;
                break
            }
        }
    }

    let mut position: usize = 0;
    let mut diff_pairs: usize = 0;
    let mut first_pair: char = ' ';

    while position < password.len() - 1 {
        if password[position] == password[position + 1]  && password[position] != first_pair {
            diff_pairs += 1;
            first_pair = password[position];
            position += 2;
        }
        else {
            position += 1;
        }
    }
    
    if straight && diff_pairs >= 2 {
        println!("Next password: {}{}{}{}{}{}{}{}", password[0], password[1], password[2], password[3], password[4], password[5], password[6], password[7]);
        true
    }
    else {
        false
    }
}

pub fn build_array(input_str: &str) -> [char; 8] {
    let temp_vec: Vec<char> = input_str.chars().collect();
    let mut out_array: [char; 8] = [' '; 8];

    for i in 0..8 {
        out_array[i] = temp_vec[i];
    }

    out_array
}