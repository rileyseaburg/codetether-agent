use super::sample;
use crate::oracle::tree_sitter_oracle::TreeSitterOracle;

#[test]
fn get_enums_finds_all() {
    let mut oracle = TreeSitterOracle::new(sample::rust_code());
    let enums = oracle.get_enums().unwrap();
    assert!(!enums.is_empty());

    let status = enums.iter().find(|e| e.name == "Status").unwrap();
    assert!(status.variants.contains(&"Active".to_string()));
    assert!(status.variants.contains(&"Inactive".to_string()));
}
