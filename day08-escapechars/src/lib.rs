pub mod characters {

    // Count the string literals in the code
    // Subtract the characters in memory
    // - Ignore leading and trailing double quotes
    // - Count escaped quotes and escapes backslashes as one character each
    // - An escaped \x followed by two hexadecimal characters counts as one character
    
    pub fn count_chars(input: &str, code_chars: &mut usize, mem_chars: &mut usize, expanded_chars: &mut usize) {
        let split_input: Vec<&str> = input.split("").collect();

        let hex_chars: [&str; 16] = [
            "0", "1", "2", "3",
            "4", "5", "6", "7",
            "8", "9", "a", "b",
            "c", "d", "e", "f"
        ];

        *code_chars += split_input.len();

        let mut counter: usize = 1;

        while counter < (split_input.len() - 1) {
            if split_input[counter] == "\\" {
                if split_input[counter + 1] == "\\" || split_input[counter + 1] == "\"" {
                    counter += 2;
                }
                else if split_input[counter + 1] == "x" && (hex_chars.contains(&split_input[counter + 2]) && hex_chars.contains(&split_input[counter + 3])) {
                    counter += 4;
                }
            }
            else {
                counter += 1;
            }
            *mem_chars += 1;
        }

        *expanded_chars += 4; // Leading and trailing double quotes

        for i in 1..split_input.len() - 1 {
            if split_input[i] == "\\" || split_input[i] == "\"" {
                *expanded_chars += 2;
            }
            else {
                *expanded_chars += 1;
            }
        }
    }
}