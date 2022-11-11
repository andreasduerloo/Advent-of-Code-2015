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