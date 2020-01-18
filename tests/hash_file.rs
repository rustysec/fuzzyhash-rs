use fuzzyhash::FuzzyHash;

#[test]
fn hash_test_data() {
    let mut fuzzy_hash = FuzzyHash::file("./tests/test_data.bin").unwrap();
    fuzzy_hash.finalize();

    assert_eq!(
        fuzzy_hash.to_string(),
        "192:tEIFoBn+SbDjIZ6MUpH6rDjHPanaVGLGOvkdGep:tEIeBrbDjRAvDVEGOMGQ".to_owned()
    );
}
