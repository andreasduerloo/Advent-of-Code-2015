use std::collections::HashMap;

#[derive(Clone)]
pub struct Location<'a> {
    pub name: &'a str,
    pub distances: Box<HashMap<&'a str,usize>>
}

impl Location<'_> {
    fn new(name: &str) -> Location {
        Location {
            name,
            distances: Box::new(HashMap::new()),
        }
    }
}

pub fn add_info<'a>(mut locations: Vec<Location<'a>>, start: &'a str, destination: &'a str, distance: usize) -> Vec<Location<'a>> {   
    if locations.len() == 0 { // We don't know anything yet
        locations.push(Location::new(start));
        locations.push(Location::new(destination));
    }

    let mut start_known: bool = false;
    let mut dest_known: bool = false;

    for i in 0..locations.len() {
        if locations[i].name == start {
            locations[i].distances.entry(destination).or_insert(distance);
            start_known = true;
        }
        else if locations[i].name == destination {
            locations[i].distances.entry(start).or_insert(distance);
            dest_known = true;
        }
    }
    if !start_known { // Would be nice to have a better Location::new that also built the HashMap
        locations.push(Location::new(start));
        for i in 0..locations.len() {
            if locations[i].name == start {
                locations[i].distances.entry(destination).or_insert(distance);
            }
        }
    }
    else if !dest_known {
        locations.push(Location::new(destination));
        for i in 0..locations.len() {
            if locations[i].name == destination {
                locations[i].distances.entry(start).or_insert(distance);
            }
        }
    }

    locations
}

pub fn next_city<'a>(current_city: Location<'a>, remaining_cities: HashMap<&'a str, Location<'a>>, route_length: usize) -> Option<(Location<'a>, HashMap<&'a str, Location<'a>>, usize)> {
    let mut out_cities = remaining_cities.clone();
    let mut out_length = route_length;
    
    if out_cities.len() == 0 {
        None
    }
    else {
        let mut closest_distance: usize = usize::MAX;
        let mut next_city: &str = "";

        for city in current_city.distances.keys() {
            if let Some(_) = out_cities.get(city) { // Check if we've been here already
                if let Some(distance) = current_city.distances.get(city) {
                    // println!("Checking distance to {} - inner loop", city);
                    if *distance < closest_distance {
                        closest_distance = *distance;
                        next_city = city;
                    }
                }       
            }
        }
        if out_cities.len() != 1 {
            // println!("Adding {} and {}", out_length, closest_distance);
            out_length += closest_distance;
        }
        // Remove the visited city
        let _ = out_cities.remove(current_city.name);

        // println!("Next up: {}", next_city);

        if out_cities.len() == 0 {
            None
        }
        else {
            Some((out_cities.get(next_city).unwrap().clone(), out_cities, out_length))
        }
    }
}

pub fn next_city_long<'a>(current_city: Location<'a>, remaining_cities: HashMap<&'a str, Location<'a>>, route_length: usize) -> Option<(Location<'a>, HashMap<&'a str, Location<'a>>, usize)> {
    let mut out_cities = remaining_cities.clone();
    let mut out_length = route_length;
    
    if out_cities.len() == 0 {
        None
    }
    else {
        let mut longest_distance: usize = 0;
        let mut next_city: &str = "";

        for city in current_city.distances.keys() {
            if let Some(_) = out_cities.get(city) { // Check if we've been here already
                if let Some(distance) = current_city.distances.get(city) {
                    // println!("Checking distance to {} - inner loop", city);
                    if *distance > longest_distance {
                        longest_distance = *distance;
                        next_city = city;
                    }
                }       
            }
        }
        if out_cities.len() != 1 {
            // println!("Adding {} and {}", out_length, closest_distance);
            out_length += longest_distance;
        }
        // Remove the visited city
        let _ = out_cities.remove(current_city.name);

        // println!("Next up: {}", next_city);

        if out_cities.len() == 0 {
            None
        }
        else {
            Some((out_cities.get(next_city).unwrap().clone(), out_cities, out_length))
        }
    }
}