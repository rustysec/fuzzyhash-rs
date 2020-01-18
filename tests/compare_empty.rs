extern crate fuzzyhash;

use fuzzyhash::FuzzyHash;

#[test]
fn compare_empty() {
    assert_eq!(FuzzyHash::compare("", ""), 0);
}
