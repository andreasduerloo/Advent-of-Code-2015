use std::collections::HashMap;

struct Location {
    distances: Box<HashMap<Box<Location>,usize>>
}