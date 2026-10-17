//! A stale frontend or executor cannot clean a replacement run's resources.

use crate::ControlPlane;

#[test]
fn cleanup_and_input_callbacks_are_scoped_to_the_current_generation() {
    let control = ControlPlane::default();
    let first = control.reserve("session").unwrap();
    let mut cleaned = false;
    assert!(control.finish_with(&first, || cleaned = true));
    assert!(cleaned);
    let second = control.reserve("session").unwrap();
    assert!(!control.finish_with(&first, || panic!("stale cleanup")));
    assert!(!control.release_prepared_with(&first, || panic!("stale release")));
    assert_eq!(
        control.with_current(&first, || panic!("stale input")),
        None::<()>
    );
    assert_eq!(
        control.with_current(&second, || "accepted"),
        Some("accepted")
    );
    assert!(control.is_current(&second));
}
