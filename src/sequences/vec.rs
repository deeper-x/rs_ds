type Grid = Vec<Pilot>;

struct Pilot {
    name: String,
}

struct Race {
    circuit: String,
    start_pilots: Grid,
    arrive_pilots: Grid,
}

impl Pilot {
    fn new(name: &str) -> Self {
        Pilot {
            name: String::from(name),
        }
    }
}

impl Grid {
    fn 
}

pub fn start_race(pilots_list: Vec<&'static str>) -> Grid {
    let grid = 
}
