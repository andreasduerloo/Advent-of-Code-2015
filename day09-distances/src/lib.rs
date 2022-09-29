use std::collections::HashMap;

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