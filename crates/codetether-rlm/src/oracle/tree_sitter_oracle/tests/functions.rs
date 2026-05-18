use super::sample;
use crate::oracle::tree_sitter_oracle::TreeSitterOracle;

#[test]
fn get_functions_finds_all() {
    let mut oracle = TreeSitterOracle::new(sample::rust_code());
    let functions = oracle.get_functions().unwrap();
    assert!(functions.len() >= 3);

    let names: Vec<&str> = functions.iter().map(|f| f.name.as_str()).collect();
    assert!(names.contains(&"new"));
    assert!(names.contains(&"with_debug"));
    assert!(names.contains(&"process"));
    assert!(names.contains(&"parse"));
}
