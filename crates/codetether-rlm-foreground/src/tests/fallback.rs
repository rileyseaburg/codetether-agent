use super::fixture::model;
use crate::selection::select;

#[test]
fn uses_caller_when_no_other_model_is_capable() {
    let models = [
        model("caller", Some(10.0), true),
        model("tiny", Some(1.0), false),
    ];
    assert_eq!(select(&models, "caller"), Some("caller"));
}
