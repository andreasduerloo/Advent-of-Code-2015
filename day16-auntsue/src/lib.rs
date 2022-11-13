pub struct Sue {
    pub number: usize,
    pub children: Option<usize>,
    pub cats: Option<usize>,
    pub samoyeds: Option<usize>,
    pub pomeranians: Option<usize>,
    pub akitas: Option<usize>,
    pub vizslas: Option<usize>,
    pub goldfish: Option<usize>,
    pub trees: Option<usize>,
    pub cars: Option<usize>,
    pub perfumes: Option<usize>
}

impl Sue {
    pub fn new(number: usize) -> Sue {
        Sue {
            number,
            cats: None,
            children: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None
        }
    }
}