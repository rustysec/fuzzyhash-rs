#![feature(test)]
extern crate test;

use fuzzyhash::FuzzyHash;
use test::Bencher;

#[bench]
fn compare_bench(b: &mut Bencher) {
    let string1 =
        "96:U57GjXnLt9co6pZwvLhJluvrszNgMFwO6MFG8SvkpjTWf:Hj3BeoEcNJ0TspgIG8SvkpjTg".to_string();
    let string2 = "96:U57GjXnLt9co6pZwvLhJluvrs1eRTxYARdEallia:Hj3BeoEcNJ0TsI9xYeia3R".to_string();

    b.iter(|| {
        FuzzyHash::compare(&string1, &string2);
    });
}
