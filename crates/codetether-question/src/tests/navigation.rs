//! navigation contract tests.
use crate::{Action, Draft, Request};
use serde_json::json;
#[test]
fn batch_back_preserves_selection_and_requires_all_answers() {
    let request = Request::parse(json!({"questions":[
        {"question":"First?", "options":[{"label":"A"},{"label":"B"}]},
        {"question":"Second?"}
    ]}))
    .unwrap();
    let mut d = Draft::new(request);
    d.apply(Action::Next);
    assert!(d.apply(Action::Submit).is_none());
    assert_eq!(d.index(), 1);
    assert!(d.apply(Action::Submit).is_none());
    d.apply(Action::Back);
    assert_eq!(d.cursor(), 1);
    d.apply(Action::Submit);
    d.apply(Action::Text("custom".into()));
    assert_eq!(
        d.apply(Action::Submit).unwrap(),
        vec![vec!["B"], vec!["custom"]]
    );
}
