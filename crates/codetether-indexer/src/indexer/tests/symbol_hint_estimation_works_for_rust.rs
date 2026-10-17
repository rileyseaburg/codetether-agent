//! Indexer behavior regression.

use super::*;

#[test]
fn symbol_hint_estimation_works_for_rust() {
    let content = "pub struct A;\nimpl A {}\nfn run() {}\n";
    assert_eq!(estimate_symbol_hints(Path::new("src/lib.rs"), content), 3);
}
