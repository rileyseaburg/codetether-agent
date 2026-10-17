//! selection contract tests.
use super::fixture::request;
use crate::{Action, Draft};

#[test]
fn single_selection_and_unicode_custom_answer() {
    let mut d = Draft::new(request(false));
    d.apply(Action::Next);
    assert_eq!(d.apply(Action::Submit).unwrap(), vec![vec!["Two"]]);
    d.apply(Action::Text("café🦀".into()));
    d.apply(Action::Backspace);
    assert_eq!(d.apply(Action::Submit).unwrap(), vec![vec!["café"]]);
}

#[test]
fn multiple_selection_and_custom_are_ordered_and_deduplicated() {
    let mut d = Draft::new(request(true));
    assert!(d.apply(Action::Submit).is_none());
    d.apply(Action::Toggle);
    d.apply(Action::Next);
    d.apply(Action::Toggle);
    d.apply(Action::Text("One".into()));
    assert_eq!(d.apply(Action::Submit).unwrap(), vec![vec!["One", "Two"]]);
    d.apply(Action::Text(" more".into()));
    assert_eq!(
        d.apply(Action::Submit).unwrap(),
        vec![vec!["One", "Two", "One more"]]
    );
}
