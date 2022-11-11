pub struct Deer<'a> {
    pub name: &'a str,
    pub speed: usize,
    pub duration: usize,
    pub rest_time: usize,
}

impl Deer<'_> {
    pub fn new(name: &str, speed: usize, duration: usize, rest_time: usize) -> Deer {
        Deer {
            name,
            speed,
            duration,
            rest_time,
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

pub fn run(deer: &Deer, duration: usize) -> usize {
    let full_block: usize = deer.duration + deer.rest_time;
    let full_blocks: usize = duration / full_block;
    let remainder: usize = duration % full_block;

    full_blocks * deer.speed * deer.duration + smallest(remainder, deer.duration) * deer.speed
}