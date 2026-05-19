use super::sample;
use crate::oracle::tree_sitter_oracle::TreeSitterOracle;

#[test]
fn get_structs_finds_all() {
    let mut oracle = TreeSitterOracle::new(sample::rust_code());
    let structs = oracle.get_structs().unwrap();
    assert!(!structs.is_empty());

    let config = structs.iter().find(|s| s.name == "Config").unwrap();
    assert!(config.fields.contains(&"debug".to_string()));
    assert!(config.fields.contains(&"timeout".to_string()));
}
