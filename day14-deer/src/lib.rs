pub struct Deer<'a> {
    pub name: &'a str,
    pub speed: usize,
    pub duration: usize,
    pub rest_time: usize,
    pub current_distance: usize,
    pub score: usize
}

impl Deer<'_> {
    pub fn new(name: &str, speed: usize, duration: usize, rest_time: usize) -> Deer {
        Deer {
            name,
            speed,
            duration,
            rest_time,
            current_distance: 0,
            score: 0,
        }
    }
}

fn smallest(first: usize, second: usize) -> usize {
    if first <= second {
        first
    }
    else {
        second
    }
}

pub fn longest(input_vec: &Vec<usize>) -> usize {
    let mut output: usize = 0;

    for i in 0..input_vec.len() {
        if input_vec[i] >= output {
            output = input_vec[i]
        }
    }

    output
}

pub fn run(deer: &Deer, duration: usize) -> usize {
    let full_block: usize = deer.duration + deer.rest_time;
    let full_blocks: usize = duration / full_block;
    let remainder: usize = duration % full_block;

    full_blocks * deer.speed * deer.duration + smallest(remainder, deer.duration) * deer.speed
}

pub fn highest_score(deer: &Vec<Deer>) -> usize { // Combine with longest() using generics?
    let mut output: usize = 0;

    for i in 0..deer.len() {
        if deer[i].score >= output {
            output = deer[i].score;
        }
    }

    output
}