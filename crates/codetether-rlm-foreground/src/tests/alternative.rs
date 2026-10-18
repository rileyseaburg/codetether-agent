use super::fixture::model;
use crate::selection::select;

#[test]
fn prefers_cheapest_capable_non_caller_model() {
    let models = [
        model("caller", Some(10.0), true),
        model("cheap", Some(1.0), true),
        model("cheaper-no-tools", Some(0.1), false),
    ];
    assert_eq!(select(&models, "caller"), Some("cheap"));
}
