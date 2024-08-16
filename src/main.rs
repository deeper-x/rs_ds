use sequences::vecdeque;

mod sequences {
    pub mod linkedlist;
    pub mod vec;
    pub mod vecdeque;
}

use sequences::linkedlist::{Ledger, Owner, Transaction};
use sequences::vec::{Pilot, Race};
use sequences::vecdeque::{Issue, TicketManager};
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

    let mut tm = TicketManager::new("tm_demo");
    let i1 = Issue::new("issue_1", false);
    let i2 = Issue::new("issue_2", true);
    let i3 = Issue::new("issue_3", false);
    let i4 = Issue::new("issue_4", true);
    let i5 = Issue::new("issue_5", false);

    tm.add_issue(i1);
    tm.add_issue(i2);
    tm.add_issue(i3);
    tm.add_issue(i4);
    tm.add_issue(i5);

    println!("{:?}", tm);
}
