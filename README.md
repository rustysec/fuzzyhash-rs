fuzzyhash-rs
============
[![Build Status](https://travis-ci.org/rustysec/fuzzyhash-rs.svg?branch=master)](https://travis-ci.org/rustysec/fuzzyhash-rs)
[![Documentation](https://docs.rs/fuzzyhash/badge.svg)](https://docs.rs/fuzzyhash)

Pure Rust fuzzy hash implementation.

### Usage

**Hash A File**
```rust
use fuzzyhash::FuzzyHash;

let fuzzy = FuzzyHash::file("/path/to/file").unwrap();

// `FuzzyHash` implements `Display` so this works:

println!("fuzzy hash of file: {}", fuzzy);
```

**Hash Data**
```rust
use fuzzyhash::FuzzyHash;

// Anything that implements `AsRef<[u8]>` can be immediately hashed

let data = vec![1,2,3,4,5,6,7,8,9,10];

let fuzzy = FuzzyHash::new(data);
```

**Anything that implements `std::io::Read`**
```rust
use fuzzyhash::FuzzyHash;
use std::io::{Cursor, Read};

let mut cursor = Cursor::new(vec![1,2,3,4,5]);
let fuzzy = FuzzyHash::read(&mut cursor);
```

**Build a fuzzy hash from blocks of data manually**
```rust
use fuzzyhash::FuzzyHash;
use std::io::Read;

let mut file = std::fs::File::open("/path/to/my/file").unwrap();
let mut fuzzy_hash = FuzzyHash::default();

loop {
    let mut buffer = vec![0; 1024];
    let count = file.read(&mut buffer).unwrap();

    fuzzy_hash.update(buffer);

if count < 1024 {
        break;
    }
}

fuzzy_hash.finalize();

println!("Fuzzy hash of data: {}", fuzzy_hash);
```

**FFI Compatibility**
Two functions provide entry points for FFI usage of this library.

```c
// hashing some data
unsigned char *data = (unsigned char*)malloc(256);
// fill this buffer...
int fuzzy = fuzzyhash(data, 256);
```

```c
// compare two fuzzyhashes
char *first = "96:U57GjXnLt9co6pZwvLhJluvrszNgMFwO6MFG8SvkpjTWf:Hj3BeoEcNJ0TspgIG8SvkpjTg";
char *second = "96:U57GjXnLt9co6pZwvLhJluvrs1eRTxYARdEallia:Hj3BeoEcNJ0TsI9xYeia3R";
int compared = fuzzyhash_compare(first, second);
```

### Status
Currently this library only supports the `None` mode of the ssdeep fuzzy hashing algorithm,
`EliminateSequences` and `DoNotTruncate` will be implemented eventually.

### Run the example
```shell
$ cargo run -q --example example1 /bin/bash
24576:z0wp2rLW2W2iYQK+q/VjsFEDe866QHX4kC:rp2rLW2W2iYJ+FEg6QHX
```
### 0.2.0 API Changes
The public API for the library has been largely re-imagined and is full of breaking changes.

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

### Acknowledgements
I previously ported the [algorithm to C++](https://github.com/rustysec/fuzzypp) and couldn't find
a version in Rust, so here we are! I definitely need to mention
[kolos450's work](https://github.com/kolos450/SsdeepNET) porting the algorithm to C#, which was
a great jumping off point for both of my implementations.
