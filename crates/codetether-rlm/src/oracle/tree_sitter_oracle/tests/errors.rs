use super::sample;
use crate::oracle::tree_sitter_oracle::TreeSitterOracle;

#[test]
fn count_error_patterns() {
    let mut oracle = TreeSitterOracle::new(sample::rust_code());
    let counts = oracle.count_error_patterns().unwrap();

    assert!(counts.result_types >= 2);
    assert!(counts.try_operators >= 1);
}
