//! Mux launch actions require both direct-process authorities.

use super::{authorized, launches_process};

#[test]
fn lifecycle_launch_is_explicitly_authorized() {
    assert!(launches_process("start"));
    assert!(launches_process("roll"));
    assert!(!launches_process("status"));
    assert!(!authorized(false, false));
    assert!(!authorized(true, false));
    assert!(!authorized(false, true));
    assert!(authorized(true, true));
}