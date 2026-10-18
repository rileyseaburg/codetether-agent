//! Regression coverage for source ambiguity, malformed language, and nullable angle.

use super::{angle, language, source};
use std::path::Path;
use windows::core::{Error, HRESULT};

#[test]
fn conflicting_source_is_rejected_without_capture() {
    let error = source::validate(Some(Path::new("image.png")), Some(42)).unwrap_err();
    assert!(error.to_string().contains("mutually exclusive"));
}

#[test]
fn independent_sources_are_accepted() {
    assert!(source::validate(Some(Path::new("image.png")), None).is_ok());
    assert!(source::validate(None, Some(42)).is_ok());
    assert!(source::validate(None, None).is_ok());
}

#[test]
fn malformed_tags_fail_before_native_language_selection() {
    for tag in ["", " ", " en-US", "en-US ", "en_US", "en\0US", "日本語"] {
        let error = language::parse(tag).unwrap_err();
        assert!(error.to_string().contains("Invalid OCR BCP-47"), "{tag:?}");
    }
}

#[test]
fn null_text_angle_is_not_an_error() {
    assert_eq!(angle::degrees(Err(Error::empty())).unwrap(), None);
}

#[test]
fn failing_text_angle_is_not_silently_null() {
    assert!(angle::degrees(Err(Error::from_hresult(HRESULT(0x80004005_u32 as i32)))).is_err());
}
