[![Build Status](https://travis-ci.org/rustysec/fuzzyhash-rs.svg?branch=master)](https://travis-ci.org/rustysec/fuzzyhash-rs)

# fuzzyhash-rs
This is a pure Rust fuzzy hash implementation!

### About
I have need of fuzzy hashing in a number of applications, and on a lot of platforms. I also did not want to rely on an external C library for the functionality.
I previously ported the [algorithm to C++](https://github.com/rustysec/fuzzypp) and couldn't find a version in Rust, so here we are!
I definitely need to mention [kolos450's work](https://github.com/kolos450/SsdeepNET) porting the algorithm to C#, which was a great jumping off point for both of my impelementations.

### Status
Currently this library only supports the "None" mode of the ssdeep fuzzy hashing algorithm, "EliminateSequences" and "DoNotTruncate" will be implemented shortly.
Also, comparing hashes is a work in progress.

* ~Simple hash output~
* ~Wire up CI~
* EliminateSequences Mode
* DoNotTruncate Mode
* ~Hash Comparisons~
* Implement tests

### Run the example
```shell
$ cargo run -q --example example1 /bin/bash
24576:z0wp2rLW2W2iYQK+q/VjsFEDe866QHX4kC:rp2rLW2W2iYJ+FEg6QHX
```
