//! Validation and editing boundaries independent of frontend rendering.
use crate::{Action, Draft, Request};
use serde_json::json;

#[test]
fn headers_count_unicode_scalars_and_labels_compare_trimmed() {
    let question = |header| json!({"questions":[{"question":"Pick", "header":header}]});
    assert!(Request::parse(question("🦀".repeat(30))).is_ok());
    assert!(Request::parse(question("🦀".repeat(31))).is_err());
    let duplicate = json!({"questions":[{"question":"Pick", "options":[
        {"label":" A "}, {"label":"A"}
    ]}]});
    assert!(Request::parse(duplicate).is_err());
}

#[test]
fn cursor_wraps_through_custom_row_without_creating_an_answer() {
    let request = Request::parse(json!({"question":"Pick", "options":["A","B"]})).unwrap();
    let mut draft = Draft::new(request);
    draft.apply(Action::Previous);
    assert_eq!(draft.cursor(), 2);
    assert!(draft.apply(Action::Submit).is_none());
    draft.apply(Action::Next);
    assert_eq!(draft.cursor(), 0);
    draft.apply(Action::Toggle);
    assert_eq!(draft.selected(), [false, false]);
    assert_eq!(draft.apply(Action::Submit).unwrap(), vec![vec!["A"]]);
}
