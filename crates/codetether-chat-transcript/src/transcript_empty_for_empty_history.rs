//! Existing export regression case.
use crate::build_transcript;
#[test]
fn transcript_empty_for_empty_history() {
    assert_eq!(build_transcript(&[]), "");
}
