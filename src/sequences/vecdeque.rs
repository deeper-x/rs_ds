use rand::distributions::{Distribution, Uniform};

use std::collections::VecDeque;

#[derive(Debug)]
pub struct Issue {
    id: String,
    is_critical: bool,
    description: String,
}

#[derive(Debug)]
pub struct TicketManager {
    id: String,
    name: String,
    issue_list: VecDeque<Issue>,
}

impl Issue {
    pub fn new(description: &str, is_critical: bool) -> Issue {
        let id = gen_id(10);
        let description_str = String::from(description);
        Issue {
            id,
            description: description_str,
            is_critical,
        }
    }
}

impl TicketManager {
    pub fn new(name: &str) -> Self {
        let id = gen_id(10);
        let issue_list = VecDeque::new();
        let name_str = String::from(name);
        TicketManager {
            id,
            name: name_str,
            issue_list,
        }
    }

    pub fn add_issue(&mut self, i: Issue) -> () {
        if !i.is_critical {
            self.issue_list.push_back(i);
        } else {
            self.issue_list.push_front(i);
        }
    }

    pub fn close_issue(&mut self, i: Issue) -> () {
        if !i.is_critical {
            self.issue_list.pop_back();
        } else {
            self.issue_list.pop_front();
        }
    }
}

fn gen_id(l: i32) -> String {
    let mut rng = rand::thread_rng();

    let mut res = String::from("ID-");
    let bucket = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let bucket_vec: Vec<char> = bucket.chars().collect();
    let bucket_len = bucket.len();

    let values = Uniform::from(0..bucket_len);

    let mut idx = 0;
    while idx <= l {
        let rand_idx = values.sample(&mut rng);
        let ch = bucket_vec[rand_idx];
        res.push(ch);
        idx += 1;
    }

    res
}
