use fuzzyhash::FuzzyHash;
use std::env;

fn main() {
    if env::args().len() < 2 {
        println!("Please provide a file to hash!");
        return;
    }

    for i in 1..env::args().len() {
        let path = env::args().nth(i).unwrap();
        let data = std::fs::read(path).expect("Could not read file");
        let fuzzy_hash = FuzzyHash::new(data);
        println!("{}", fuzzy_hash.to_string());
    }
}
