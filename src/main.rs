mod sequences {
    pub mod vec;
}

use sequences::vec::{Pilot, Race};

fn main() {
    let p1 = Pilot::new("capirossi", "ducati");
    let p2 = Pilot::new("cadalora", "Yamaha");
    let p3 = Pilot::new("rossi", "Honda");

    let mut r1 = Race::new("Misano");

    r1.start_add_pilot(p1);
    r1.start_add_pilot(p2);
    r1.start_add_pilot(p3);

    println!("{}", r1.warmup());

    println!("{:?}", r1.run_race());
}
