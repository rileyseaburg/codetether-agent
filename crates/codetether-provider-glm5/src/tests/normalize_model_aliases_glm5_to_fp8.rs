//! Regression coverage for normalize_model_aliases_glm5_to_fp8.
use crate::*;

#[test]
fn normalize_model_aliases_glm5_to_fp8() {
    assert_eq!(Glm5Provider::normalize_model("glm-5"), DEFAULT_MODEL);
    assert_eq!(Glm5Provider::normalize_model("glm5"), DEFAULT_MODEL);
    assert_eq!(Glm5Provider::normalize_model(""), DEFAULT_MODEL);
    assert_eq!(Glm5Provider::normalize_model("glm5/glm-5"), DEFAULT_MODEL);
    assert_eq!(Glm5Provider::normalize_model("glm5:glm-5"), DEFAULT_MODEL);
}
