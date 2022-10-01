pub fn play_game(input: Vec<char>, iterations: usize) -> usize {
    match iterations {
        0 => {
            input.len()
        }
        1 => {
            let result_vec = single_round(input);
            result_vec.len()
        }
        _ => {
            let mut intermediate_vec = single_round(input);

            for _ in 1..iterations {
                intermediate_vec = single_round(intermediate_vec);
            }
            intermediate_vec.len()
        }
    }
}

fn single_round(input: Vec<char>) -> Vec<char> {
    let mut out_vec: Vec<char> = Vec::new();

    let mut position: usize = 0;

    while position < input.len() {
        let character = input[position];
        let number = run_length(&character, &input, &position);
        position += number;
        let number_as_str = number.to_string();
        let num_vec: Vec<char> = number_as_str.chars().collect();

        for letter in num_vec {
            out_vec.push(letter);
        }
        out_vec.push(character);
    }
    
    // for i in 0..input.len() {
    //     let character = input[i];
    //     let number = run_length(&character, &input, &i);
    //     let number_as_str = number.to_string();
    //     let num_vec: Vec<char> = number_as_str.chars().collect();

    //     for letter in num_vec {
    //         out_vec.push(letter);
    //     }
    //     out_vec.push(character);
    // }

    out_vec
}

fn run_length(character: &char, word: &Vec<char>, start: &usize) -> usize {
    let mut count: usize = 1;

    for i in (*start+1)..word.len() {
        if word[i] == *character {
            count += 1;
        }
        else {
            break
        }
    }
    count
}