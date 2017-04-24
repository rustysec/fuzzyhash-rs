extern crate fuzzyhash;

use fuzzyhash::compare::{strings};

#[test]
fn compare1() {
    assert_eq!(
        strings(
            "96:U57GjXnLt9co6pZwvLhJluvrszNgMFwO6MFG8SvkpjTWf:Hj3BeoEcNJ0TspgIG8SvkpjTg".to_string(),
            "96:U57GjXnLt9co6pZwvLhJluvrs1eRTxYARdEallia:Hj3BeoEcNJ0TsI9xYeia3R".to_string()),
        63);
}
