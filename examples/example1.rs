extern crate fuzzyhash;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    if env::args().len() < 2 {
        println!("Please provide a file to hash!");
        return;
    }

    for i in 1..env::args().len() {
        let path = env::args().nth(i).unwrap();
        match File::open(&path) {
            Ok(mut f) => {
                let mut buffer = Vec::new();
                match f.read_to_end(&mut buffer) {
                    Ok(_) => {
                        println!("{}", fuzzyhash::hash_buffer(buffer));
                    }
                    Err(e_read) => {
                        println!("Could not read '{}': {}", path, e_read);
                    }
                }
            }
            Err(e) => {
                println!("Cannot open '{}': {}", path, e);
            }
        }
    }
}
