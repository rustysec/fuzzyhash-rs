use fuzzyhash::FuzzyHash;

#[test]
fn hash_test_data() {
    let fuzzy_hash = FuzzyHash::file("./tests/test_data.bin").unwrap();

    assert_eq!(
        fuzzy_hash.to_string(),
        "192:tEIFoBn+SbDjIZ6MUpH6rDjHPanaVGLGOvkdGep:tEIeBrbDjRAvDVEGOMGQ".to_owned()
    );
}
