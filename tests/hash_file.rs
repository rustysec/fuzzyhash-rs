use fuzzyhash::hash_file;

#[test]
fn hash_test_data() {
    use std::path::PathBuf;
    assert_eq!(
        hash_file(PathBuf::from("./tests/test_data.bin")).unwrap(),
        "192:tEIFoBn+SbDjIZ6MUpH6rDjHPanaVGLGOvkdGep:tEIeBrbDjRAvDVEGOMGQ".to_owned()
    );
}
