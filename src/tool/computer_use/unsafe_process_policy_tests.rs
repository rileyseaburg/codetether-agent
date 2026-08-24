//! Windows direct transport requires all independent authorities.

use super::{authorized, signed_network};

#[test]
fn unsafe_process_requires_setting_approval_and_network() {
    assert!(!authorized(false, false, false));
    assert!(!authorized(true, false, true));
    assert!(!authorized(false, true, true));
    assert!(!authorized(true, true, false));
    assert!(authorized(true, true, true));
}

#[test]
fn ambient_network_cannot_authorize_unsandboxed_powershell() {
    let _lock = crate::approval::test_env::lock_env();
    let _network = crate::tool::network_access::test_env::Network::set("1");
    let mut args = serde_json::json!({
        "__ct_session_id": "windows-test",
        "__ct_parent_workspace": "workspace"
    });
    assert!(!signed_network(&args));
    crate::tool::network_access::bind_trusted(&mut args, true);
    assert!(signed_network(&args));
}
