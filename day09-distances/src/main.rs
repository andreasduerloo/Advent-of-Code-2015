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

    // Turn the vector of locations into a HashMap
    let mut location_map: HashMap<&str, Location> = HashMap::new();

    for location in locations {
        location_map.insert(location.name, location);
    }

    let mut shortest_route: usize = usize::MAX;
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

        //println!("Route length for the route starting in {} is {}.", starting_city, route_length);
        if route_length < shortest_route {
            shortest_route = route_length;
        }
    }

    println!("First star: the shortest route is {} units long.", shortest_route);

    // Now we do it all again for the second star

    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let input_vec: Vec<&str> = input.lines().collect();

    let mut locations: Vec<Location> = Vec::new();

    for line in input_vec {
        let elements: Vec<&str> = line.split_whitespace().collect();
        let distance: usize = usize::from_str_radix(elements[4], 10).unwrap();
        locations = add_info(locations, elements[0], elements[2], distance);
    }

    // Turn the vector of locations into a HashMap
    let mut location_map: HashMap<&str, Location> = HashMap::new();

    for location in locations {
        location_map.insert(location.name, location);
    }

    let mut longest_route: usize = 0;
    // Start with each of the eight cities, always go to the nearest, unvisited city

    for starting_city in location_map.keys() {
        // println!("Testing {}", starting_city);
        let local_map = location_map.clone();
        let current_city: Location = local_map.get(starting_city).unwrap().clone();
        let mut route_length: usize = 0;

        let mut context: Option<(Location, HashMap<&str, Location>, usize)> = next_city_long(current_city, local_map, route_length);

        while let Some(output) = context {
            let city = output.0;
            let map = output.1.clone();
            let distance = output.2;

            route_length = output.2;
            context = next_city_long(city, map, distance);
        }

        //println!("Route length for the route starting in {} is {}.", starting_city, route_length);
        if route_length > longest_route {
            longest_route = route_length;
        }
    }

    println!("Second star: the longest route is {} units long.", longest_route);
}
