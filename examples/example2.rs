extern crate fuzzyhash;

use std::env;
use fuzzyhash::compare::strings;

pub fn main() {
    if env::args().len() != 3 {
        println!("Must provide two hashes!");
        return;
    }
    println!("first: {}", env::args().nth(1).unwrap());
    println!("second: {}", env::args().nth(2).unwrap());

    let s = strings(env::args().nth(1).unwrap(), env::args().nth(2).unwrap());

    println!("Strings are {}% similar!", s);
}
