use sequences::vec;

mod sequences {
    pub mod vec;
}

fn main() {
    let pilots = vec::get_pilots_list();

    println!("Starting list: {:?}", pilots);
}
