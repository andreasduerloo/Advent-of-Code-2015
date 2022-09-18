pub mod lights {
    pub fn turn_off(grid: &mut[[bool; 1000]; 1000], points: &[[usize; 2]; 2]) {
        //
    }

    pub fn turn_on(grid: &mut[[bool; 1000]; 1000], points: &[[usize; 2]; 2]) {
        //
    }

    pub fn toggle(grid: &mut[[bool; 1000]; 1000], points: &[[usize; 2]; 2]) {
        //
    }
}

pub mod input {
    pub enum Command {
        Off,
        On,
        Toggle
    }

    pub fn parse_line(input: &str) -> Option<(Command, [[usize; 2]; 2])> {
        let input_vec: Vec<&str> = input.split(" ").collect();

        // Valid lengths are 4 and 5
        match input_vec.len() {
            4 => {
                // This is a toggle
                let raw_points: [&str; 2] = [input_vec[1], input_vec[3]];
                let points = normalize_coordinates(raw_points);
                Some((Command::Toggle, points))
            },
            5 => {
                // This is either turn on or turn off
                match input_vec[1] {
                    "off" => {
                        let raw_points: [&str; 2] = [input_vec[2], input_vec[4]];
                        let points = normalize_coordinates(raw_points);
                        Some((Command::Off, points))
                    },
                    "on" => {
                        let raw_points: [&str; 2] = [input_vec[2], input_vec[4]];
                        let points = normalize_coordinates(raw_points);
                        Some((Command::On, points))
                    },
                    _ => None
                }

            },
            _ => None
        }
    }

    fn normalize_coordinates(input: [&str; 2]) -> [[usize; 2]; 2] {
        // Return the bottom-left and top-right coordinates for a rectangle
        // Input takes this form: 205,417
        let first_vec: Vec<&str> = input[0].split(",").collect();
        let second_vec: Vec<&str> = input[1].split(",").collect();

        let mut first_point: [usize; 2] = [0, 0];
        let mut second_point: [usize; 2] = [0, 0];

        first_point[0] = usize::from_str_radix(first_vec[0], 10).unwrap();
        first_point[1] = usize::from_str_radix(first_vec[1], 10).unwrap();
        second_point[0] = usize::from_str_radix(second_vec[0], 10).unwrap();
        second_point[1] = usize::from_str_radix(second_vec[1], 10).unwrap();

        let mut output: [[usize; 2]; 2] = [[0; 2]; 2];

        // Build the bottom-left corner
        output[0][0] = first_point[0];
        if second_point[0] < first_point[0] {
            output[0][0] = second_point[0];
        }

        output[0][1] = first_point[1];
        if second_point[1] < first_point[1] {
            output[0][1] = second_point[1];
        }

        // Build the top-right corner
        output[1][0] = first_point[0];
        if second_point[0] > first_point[0] {
            output[1][0] = second_point[0];
        }

        output[1][1] = first_point[1];
        if second_point[1] > first_point[1] {
            output[1][1] = second_point[1];
        }
        output
    }
}