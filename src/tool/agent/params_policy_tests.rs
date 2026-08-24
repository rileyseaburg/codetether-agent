//! Signed network inheritance through legacy agent parameters.

use serde_json::json;

#[test]
fn resume_config_accepts_only_signed_parent_network_policy() {
    let mut args = json!({
        "action":"spawn", "__ct_session_id":"parent",
        "__ct_parent_workspace":"/workspace",
    });
    crate::tool::network_access::bind_trusted(&mut args, true);
    let params: super::super::params::Params = serde_json::from_value(args.clone()).unwrap();
    assert_eq!(params.resume_config().network_allowed, Some(true));

    args["__ct_parent_workspace"] = json!("/retargeted");
    let retargeted: super::super::params::Params = serde_json::from_value(args).unwrap();
    assert_eq!(retargeted.resume_config().network_allowed, None);
}