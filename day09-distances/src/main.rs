use std::fs;
use day09_distances::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let input_vec: Vec<&str> = input.lines().collect();

    let mut locations: Vec<Location> = Vec::new();

    for line in input_vec {
        let elements: Vec<&str> = line.split_whitespace().collect();
        let distance: usize = usize::from_str_radix(elements[4], 10).unwrap();
        locations = add_info(locations, elements[0], elements[2], distance);
    }

    // println!("These are the cities we know, along with how many cities they know the distance to:");
    // for location in locations {
    //     println!("Name: {} - known distances: {}", location.name, location.distances.len());
    // }

    // println!("These are the distances from Tristram:");
    // for distance in locations[0].distances.keys() {
    //     println!("City: {}, distance: {}", distance, locations[0].distances.get(distance).unwrap());
    // }

    let mut shortest_route: usize = usize::MAX;
    // Start with each of the eight cities, always follow the shortest path

    for i in locations.length() {
        let mut route_length: usize 0;
        let mut visited_cities: Vec<&str> = Vec::new();
        let current_city = locations[i];

        while Some(city) = next_city(&current_city, &mut visited_cities, &locations) {
            route_length += locations
        }
    }
}
