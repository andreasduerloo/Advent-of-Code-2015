pub mod filter {
    use std::collections::HashMap;

    pub fn first_pass<'a>(input_vec: Vec<&'a str>, wires: &mut HashMap<&'a str, u16>) -> Vec<Vec<&'a str>> {
        let mut output_vec: Vec<Vec<&str>> = Vec::new();

        for instruction in input_vec { // The iterator will consume the vector
            let split: Vec<&str> = instruction.split_whitespace().collect();

            if split.len() == 3 { // These are the only lines with three elements
                // We're only interested in those lines where the first element is a u16

                if let Ok(some_u16) = u16::from_str_radix(split[0], 10) {
                    wires.entry(split[2]).or_insert(some_u16);
                }
                else {
                    output_vec.push(split); // format wire -> wire, we'll need this later
                }
            }
            else {
                output_vec.push(split);
            }
        }
        output_vec
    }

    pub fn pass<'a>(input_vec: Vec<Vec<&'a str>>, wires: &mut HashMap<&'a str, u16>) -> Vec<Vec<&'a str>> {
        let mut output_vec: Vec<Vec<&str>> = Vec::new();

        for instruction in input_vec {
            match instruction.len() {
                3 => { // This is "wire -> wire"
                    if let Some(value) = wires.get(instruction[0]) { // Do we know the value for the left wire?
                        let wire_value: u16 = *value;
                        wires.entry(instruction[2]).or_insert(wire_value);
                    }
                    else {
                        output_vec.push(instruction);
                    }
                },
                4 => { // This is "NOT wire -> wire"
                    if let Some(value) = wires.get(instruction[1]) { // Do we know the value for input wire?
                        let wire_value: u16 = *value;
                        wires.entry(instruction[3]).or_insert(!wire_value);
                    }
                    else {
                        output_vec.push(instruction);
                    }
                },
                5 => { // This is everything else (AND, OR, LSHIFT, RSHIFT)
                    match instruction[1] {
                        "LSHIFT" => {
                            if let Some(value) = wires.get(instruction[0]) {
                                if let Ok(shift) = usize::from_str_radix(instruction[2], 10) {
                                    let wire_value: u16 = *value << shift;
                                    wires.entry(instruction[4]).or_insert(wire_value);
                                }
                            }
                            else {
                                output_vec.push(instruction);
                            }
                        },
                        "RSHIFT" => {
                            if let Some(value) = wires.get(instruction[0]) {
                                if let Ok(shift) = usize::from_str_radix(instruction[2], 10) {
                                    let wire_value: u16 = *value >> shift;
                                    wires.entry(instruction[4]).or_insert(wire_value);
                                }
                            }
                            else {
                                output_vec.push(instruction);
                            }
                        },
                        "AND" => {
                            if let Some(value) = wires.get(instruction[0]) { // First element is a known wire
                                if let Some(other_value) = wires.get(instruction[2]) { // So is the second element
                                    let wire_value: u16 = *value & *other_value;
                                    wires.entry(instruction[4]).or_insert(wire_value);
                                }
                                else if let Ok(other_value) = u16::from_str_radix(instruction[2], 10) { // Second element is a number
                                    let wire_value: u16 = *value & other_value;
                                    wires.entry(instruction[4]).or_insert(wire_value);
                                }
                                else {
                                    output_vec.push(instruction);
                                }
                            }
                            else if let Ok(value) = u16::from_str_radix(instruction[0], 10) { // First element is a number
                                if let Some(other_value) = wires.get(instruction[2]) { // Second element is a known wire
                                    let wire_value: u16 = value & *other_value;
                                    wires.entry(instruction[4]).or_insert(wire_value);
                                }
                                else if let Ok(other_value) = u16::from_str_radix(instruction[2], 10) { // Second element is also a number
                                    let wire_value: u16 = value & other_value;
                                    wires.entry(instruction[4]).or_insert(wire_value);
                                }
                                else {
                                    output_vec.push(instruction);
                                } 
                            }
                            else {
                                output_vec.push(instruction);
                            }
                        },
                        "OR" => {
                            if let Some(value) = wires.get(instruction[0]) { // First element is a known wire
                                if let Some(other_value) = wires.get(instruction[2]) { // So is the second element
                                    let wire_value: u16 = *value | *other_value;
                                    wires.entry(instruction[4]).or_insert(wire_value);
                                }
                                else if let Ok(other_value) = u16::from_str_radix(instruction[2], 10) { // Second element is a number
                                    let wire_value: u16 = *value | other_value;
                                    wires.entry(instruction[4]).or_insert(wire_value);
                                }
                                else {
                                    output_vec.push(instruction);
                                }
                            }
                            else if let Ok(value) = u16::from_str_radix(instruction[0], 10) { // First element is a number
                                if let Some(other_value) = wires.get(instruction[2]) { // Second element is a known wire
                                    let wire_value: u16 = value | *other_value;
                                    wires.entry(instruction[4]).or_insert(wire_value);
                                }
                                else if let Ok(other_value) = u16::from_str_radix(instruction[2], 10) { // Second element is also a number
                                    let wire_value: u16 = value | other_value;
                                    wires.entry(instruction[4]).or_insert(wire_value);
                                }
                                else {
                                    output_vec.push(instruction);
                                } 
                            }
                            else {
                                output_vec.push(instruction);
                            }
                        },
                        _ => continue
                    }
                },
                _ => continue // Shouldn't occur, whatever it is we're not copying it to the new vector
            }
        }
        output_vec   
    }
}