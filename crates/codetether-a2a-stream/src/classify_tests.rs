//! Regression cases for the parent stream module.

use super::{EventClass, classify};

#[test]
fn known_types() {
    assert_eq!(classify("task_available"), EventClass::Advisory);
    assert_eq!(classify("connected"), EventClass::Control);
    assert_eq!(classify("resync-required"), EventClass::Control);
    assert_eq!(classify("progress"), EventClass::Sequenced);
}

#[test]
fn unknown_defaults_sequenced() {
    assert_eq!(classify("brand_new_event"), EventClass::Sequenced);
}
