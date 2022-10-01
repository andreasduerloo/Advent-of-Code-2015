use std::fs;
use day09_distances::*;
use std::collections::HashMap;

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

    // Turn the vector of locations into a HashMap
    let mut location_map: HashMap<&str, Location> = HashMap::new();

    for location in locations {
        location_map.insert(location.name, location);
    }

    // let mut shortest_route: usize = usize::MAX;
    // Start with each of the eight cities, always go to the nearest, unvisited city

    for starting_city in location_map.keys() {
        // println!("Testing {}", starting_city);
        let local_map = location_map.clone();
        let current_city: Location = local_map.get(starting_city).unwrap().clone();
        let mut route_length: usize = 0;

        let mut context: Option<(Location, HashMap<&str, Location>, usize)> = next_city(current_city, local_map, route_length);

        while let Some(output) = context {
            let city = output.0;
            let map = output.1.clone();
            let distance = output.2;

            route_length = output.2;
            context = next_city(city, map, distance);
        }

        println!("Route length for the route starting in {} is {}.", starting_city, route_length);
    }
}
