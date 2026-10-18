//! Existing substitution behavior and unknown-command preservation.
use crate::prettify_math;

#[test]
fn prettify_math_substitutes_known_symbols() {
    assert_eq!(prettify_math("\\sum_{i=1}^n"), "Σ_{i=1}^n");
    assert_eq!(prettify_math("\\delta_{st}"), "δ_{st}");
    assert_eq!(
        prettify_math("H_n=(\\mathbb C)^{\\otimes n}"),
        "H_n=(ℂ)^{⊗ n}"
    );
    // Unknown commands pass through unchanged.
    assert_eq!(prettify_math("\\unknownmacro x"), "\\unknownmacro x");
}
