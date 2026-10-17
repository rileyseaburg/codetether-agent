//! Regression coverage for normalize_model_strips_prefix.
use crate::*;

#[test]
fn normalize_model_strips_prefix() {
    assert_eq!(Glm5Provider::normalize_model("glm5/glm-5-fp8"), "glm-5-fp8");
    assert_eq!(Glm5Provider::normalize_model("glm5:glm-5-fp8"), "glm-5-fp8");
    assert_eq!(Glm5Provider::normalize_model("glm-5-fp8"), "glm-5-fp8");
}
