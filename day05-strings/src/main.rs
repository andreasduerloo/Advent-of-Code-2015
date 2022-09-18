/*
First star
A string is NICE when:
- It contains three or more vowels (aeiou)
- It contains at least one letter that appears twice in a row
- It does NOT contain:
  - ab
  - cd
  - pq
  - xy
---
Second star
A string is NICE when:
- A pair of letters appears twice, without overlapping
- It contains a letter that repeats with exactly one other letter in between
*/

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let input_vec: Vec<&str> = input.lines().collect();

    let mut first_nice_count: usize = 0;
    let mut second_nice_count: usize = 0;

    for word in input_vec {
        if check_first_niceness(&word) {
            first_nice_count += 1;
        }
        if check_second_niceness(&word) {
            second_nice_count += 1;
        }
    }

    println!("First star, nice words: {}", first_nice_count);
    println!("Second star, nice words: {}", second_nice_count);
}

fn check_first_niceness(input: &str) -> bool {
    let char_vec: Vec<char> = input.chars().collect();

    let bad_slices = [
        "ab",
        "cd",
        "pq",
        "xy"
    ];

    let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    // Check if it contains any of the forbidden combinations
    for i in 0..char_vec.len() - 1 {
        for slice in bad_slices {
            let slice_chars: Vec<char> = slice.chars().collect();
            if char_vec[i] == slice_chars[0] && char_vec[i+1] == slice_chars[1] {
                return false;
            }
        }
    }

    // Check for a letter that appears twice in a row
    let mut double_found: bool = false;
    for i in 0..char_vec.len() - 1 {
        if char_vec[i] == char_vec[i+1] {
            double_found = true;
            break
        }
    }

    // Check for at least three vowels
    let mut vowel_check: bool = false;
    let mut vowel_count: u8 = 0;
    for character in char_vec {
        if vowels.contains(&character) {
            vowel_count += 1;
        }
    }
    if vowel_count > 2 {
        vowel_check = true;
    }

    return double_found & vowel_check
}

fn check_second_niceness(input: &str) -> bool {
    let char_vec: Vec<char> = input.chars().collect();

    let mut doubles_check: bool = false;
    for i in 0..char_vec.len() - 3 {
        for j in i+2..char_vec.len() - 1 {
            if char_vec[i] == char_vec[j] && char_vec[i + 1] == char_vec[j + 1] {
                doubles_check = true;
            }
        }
    }

    let mut mirror_check: bool = false;
    for i in 0..char_vec.len() - 2 {
        if char_vec[i] == char_vec[i + 2] {
            mirror_check = true;
        }
    }

    return doubles_check & mirror_check
}