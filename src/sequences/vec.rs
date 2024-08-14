use rand::distributions::{Distribution, Uniform};

pub type Grid = Vec<Pilot>;

#[derive(Debug)]
pub struct Pilot {
    name: String,
    moto: Motorbike,
}

#[derive(Debug)]
pub struct Motorbike {
    name: String,
}

#[derive(Debug)]
pub struct Race {
    circuit: String,
    start: Grid,
    finish: Grid,
}

impl Pilot {
    pub fn new(name: &str, moto: &str) -> Self {
        Pilot {
            name: String::from(name),
            moto: Motorbike {
                name: String::from(moto),
            },
        }
    }
}

impl Race {
    pub fn new(c_name: &str) -> Self {
        let circuit_name = String::from(c_name);

        Race {
            circuit: circuit_name,
            start: vec![],
            finish: vec![],
        }
    }

    pub fn start_add_pilot(&mut self, p: Pilot) -> () {
        self.start.push(p);
    }

    pub fn finish_add_pilot(&mut self, p: Pilot) -> () {
        self.finish.push(p)
    }

    pub fn finish_race(&self) -> Grid {
        let mut res = Grid::new();
        let tot = self.start.len();
        let uf = Uniform::from(0..tot);
        let mut rng = rand::thread_rng();

        let mut wo_issues: Vec<usize> = vec![];

        for p in &self.start {
            let cur: usize = uf.sample(&mut rng);

            if !wo_issues.contains(&cur) {
                let this_p: Pilot = Pilot::new(&p.name, &p.moto.name);

                wo_issues.push(cur);
                res.push(this_p);
            }
        }

        res
    }

    pub fn dump_starting_grid(&self) -> String {
        let mut res = "Starting line:\n".to_string();

        for v in &self.start {
            let cur = format!("Pilot {} - Moto {}\n", v.name, v.moto.name);
            res.push_str(&cur)
        }

        res.to_string()
    }
}
