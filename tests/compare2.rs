extern crate fuzzyhash;

use fuzzyhash::compare::{strings};

#[test]
fn compare2() {
    assert_eq!(
        strings(
            "".to_string(),
            "".to_string()),
        0);
}
