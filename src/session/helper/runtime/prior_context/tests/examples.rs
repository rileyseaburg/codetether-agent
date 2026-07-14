use super::user;
use crate::session::helper::runtime::prior_context_allowed;

#[test]
fn quoted_docs_and_list_examples_are_not_directives() {
    for text in [
        "fix the test containing \"its in the docs\"",
        "rename the `documentation is the source of truth` fixture",
    ] {
        assert!(prior_context_allowed(&[user(text)]), "{text}");
    }
    let messages = [
        user("never use session recall"),
        user("examples:\nyou can use session recall again\n1. enable memory"),
    ];
    assert!(!prior_context_allowed(&messages));
}
