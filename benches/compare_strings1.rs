#![feature(test)]
extern crate fuzzyhash;
extern crate test;

use fuzzyhash::compare::{strings};
use test::Bencher;

#[bench]
fn compare_bench(b: &mut Bencher) {
    let string1 = "96:U57GjXnLt9co6pZwvLhJluvrszNgMFwO6MFG8SvkpjTWf:Hj3BeoEcNJ0TspgIG8SvkpjTg".to_string();
    let string2 = "96:U57GjXnLt9co6pZwvLhJluvrs1eRTxYARdEallia:Hj3BeoEcNJ0TsI9xYeia3R".to_string();
    
    b.iter(|| {
        let mut i = 0;
        while i < 1000 {
            let _comp = strings(
                string1.to_owned(),
                string2.to_owned()
            );
            i += 1;
        }
    });
}
