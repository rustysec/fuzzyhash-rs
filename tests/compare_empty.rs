extern crate fuzzyhash;

use fuzzyhash::FuzzyHash;

#[test]
fn compare_empty() {
    assert!(FuzzyHash::compare("", "").is_err());
}
