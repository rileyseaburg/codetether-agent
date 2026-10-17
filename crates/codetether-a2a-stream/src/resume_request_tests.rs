//! Regression cases for the parent stream module.

use super::EventId;
use super::apply_resume;

#[test]
fn none_returns_buildable_request() {
    let client = reqwest::Client::new();
    let req = apply_resume(client.get("http://localhost/x"), None);
    assert!(req.build().is_ok());
}

#[test]
fn some_sets_header() {
    let client = reqwest::Client::new();
    let id = EventId {
        epoch: "ep".into(),
        seq: 5,
    };
    let req = apply_resume(client.get("http://localhost/x"), Some(&id));
    let built = req.build().unwrap();
    assert_eq!(built.headers().get("Last-Event-ID").unwrap(), "ep.5");
}
