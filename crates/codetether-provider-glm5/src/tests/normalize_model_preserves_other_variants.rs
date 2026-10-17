//! Regression coverage for normalize_model_preserves_other_variants.
use crate::*;

#[test]
fn normalize_model_preserves_other_variants() {
    assert_eq!(
        Glm5Provider::normalize_model("glm5/glm-5-int4"),
        "glm-5-int4"
    );
}
