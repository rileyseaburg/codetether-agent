//! workflow_progress regression.
use super::*;

#[test]
fn progress_boundaries_and_validation_order() {
    let mut okr = Okr::new("", "description");
    assert!(matches!(
        okr.validate(),
        Err(OkrValidationError::EmptyTitle)
    ));
    okr.title = "valid".into();
    assert!(matches!(
        okr.validate(),
        Err(OkrValidationError::NoKeyResults)
    ));
    assert_eq!(okr.progress(), 0.0);
    assert!(okr.is_complete());
    let mut kr = KeyResult::new(okr.id, "goal", 10.0, "count");
    kr.update_progress(-1.0);
    assert_eq!(kr.progress(), 0.0);
    kr.update_progress(15.0);
    assert_eq!(kr.progress(), 1.0);
    kr.update_progress(1.0);
    assert!(kr.is_complete());
    okr.add_key_result(kr);
    assert!(okr.validate().is_ok());
    assert_eq!(
        OkrValidationError::NotApproved.to_string(),
        "run must be approved before starting"
    );
}
