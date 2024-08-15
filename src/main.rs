mod sequences {
    pub mod linkedlist;
    pub mod vec;
}

use sequences::linkedlist::{Ledger, Owner, Transaction};
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

    let o1 = Owner::new("demo 1");
    let o2 = Owner::new("demo 2");

    let t1 = Transaction::new(o1, "IN", 200.12);
    let t2 = Transaction::new(o2, "OUT", 100.23);

    let mut l = Ledger::new();

    l.add_transaction(t1);
    l.add_transaction(t2);

    println!("{:?}", l);
}
