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

### 0.1.3 Updates
Fixed performance bottlenecks with cloning large buffers unnecessarily (~22% faster).

1000 iterations of large random buffer

0.1.2:
```sh
$ time cargo bench
    Finished release [optimized] target(s) in 0.0 secs
     Running target/release/deps/fuzzyhash-a709fbd8d1125c4f

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/release/deps/random_data1-6d3edf5ebe8a1b5f

running 1 test
test hashing_bench ... bench: 111,144,101 ns/iter (+/- 2,712,598)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out


real    0m33.786s
user    0m33.757s
sys     0m0.030s
```

vs

0.1.3:
```sh
$ time cargo bench
    Finished release [optimized] target(s) in 0.0 secs
     Running target/release/deps/fuzzyhash-9ad0dfdb1b3b0386

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/release/deps/random_data1-3bec1fdd42a47a95

running 1 test
test hashing_bench ... bench:  87,273,582 ns/iter (+/- 2,535,966)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out


real    0m26.525s
user    0m26.515s
sys     0m0.011s
```

