extern crate fuzzyhash;

use fuzzyhash::compare::{strings};

#[test]
fn compare_empty() {
    assert_eq!(
        strings(
            "".to_string(),
            "".to_string()),
        0);
}
