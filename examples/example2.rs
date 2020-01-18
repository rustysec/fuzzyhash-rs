use fuzzyhash::FuzzyHash;
use std::env;

pub fn main() {
    if env::args().len() != 3 {
        println!("Must provide two hashes!");
        return;
    }
    println!("first: {}", env::args().nth(1).unwrap());
    println!("second: {}", env::args().nth(2).unwrap());

    let first = FuzzyHash::from(env::args().nth(1).unwrap());
    let second = FuzzyHash::from(env::args().nth(2).unwrap());

    println!(
        "Strings are {}% similar!",
        first.compare_to(&second).unwrap_or(0)
    );
}
