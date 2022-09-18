use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let mut input_vec: Vec<&str> = input.lines().collect();

    let mut element = get_dimensions(input_vec.pop());
    let mut total_surface: usize = 0;

    while let Some(dimensions) = element {
        total_surface += get_surface(&dimensions);
        element = get_dimensions(input_vec.pop());
    }

    println!("You should order {} square feet of wrapping paper. 🎁", total_surface);
}

fn get_surface(dimensions: &[usize; 3]) -> usize {
    // Build an array of products
    let areas: [usize; 3] = [dimensions[0] * dimensions[1], dimensions[0] * dimensions[2], dimensions[1] * dimensions[2]];

    // Get the smallest surface
    let margin = smallest_side(&areas);

    // Sum and double the values in the array
    let mut output = 0;

    for area in areas {
        output += area * 2;
    }

    // Add the smallest value
    output += margin;
    output
}

fn smallest_side(areas: &[usize; 3]) -> usize {
    let mut minimum = areas[0];

    for area in areas {
        if minimum > *area {
            minimum = *area;
        }
    }
    minimum
}

fn get_dimensions(input: Option<&str>) -> Option<[usize; 3]> {
    if let Some(value) = input {
        let sides: Vec<&str> = value.split("x").collect();
        match sides.len() {
            3 => {
                let mut out_array: [usize; 3] = [0, 0, 0];

                for i in 0..sides.len() {
                    out_array[i] = usize::from_str_radix(sides[i], 10).unwrap();
                }

                Some(out_array)
            },
            _ => None
        }
    }    
    else {
        None
    }
}
