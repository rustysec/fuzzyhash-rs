extern crate fuzzyhash;

#[test]
pub fn fix_breakage() {
    let first = "3072:oQGiMXTMkux9BPSd0n4bmzwuy+WAAux3i8:op1XTsbBBnnU8nAu48";
    let second = "3072:zszq392p8xWp9+fbhBpmLOCeTFvm7RAkEmq8RPFc21xgpYn9R:Agse0Yb//hu7RAkc87go9";
    assert_eq!(fuzzyhash::compare::strs(first,second), 0);
}
