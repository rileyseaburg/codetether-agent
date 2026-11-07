//! Runtime envelopes are not part of the public Monarch argument schema.
use super::input::{Input, parse};
use serde_json::{Value, json};

fn enriched(action: &str) -> Value {
    json!({"action":action,"__ct_agent_identity_id":"fixture-agent",
        "__ct_session_id":"fixture-session","__ct_agent_name":"build",
        "__ct_origin":"swarm","__ct_provenance_id":"fixture-provenance",
        "__ct_parent_workspace":"/fixture","__ct_current_model":"fixture/model",
        "__ct_prior_context_allowed":false,"__ct_lease_owner":"fixture-turn",
        "_tool_call_id":"fixture-call"})
}

#[test]
fn accepts_plain_and_runtime_enriched_actions() {
    assert!(matches!(
        parse(json!({"action":"list_tools"})),
        Ok(Input::ListTools {})
    ));
    assert!(matches!(
        parse(enriched("list_tools")),
        Ok(Input::ListTools {})
    ));
    assert!(matches!(parse(enriched("status")), Ok(Input::Status {})));
}

#[test]
fn retains_nested_remote_arguments_without_treating_them_as_runtime_fields() {
    let arguments = json!({"__ct_agent_identity_id":"remote-field",
        "_tool_call_id":"remote-call","nested":{"__ct_origin":"remote-origin"}});
    let mut input = enriched("call");
    input["tool"] = json!("get_accounts");
    input["arguments"] = arguments.clone();
    match parse(input).unwrap() {
        Input::Call {
            tool,
            arguments: actual,
        } => {
            assert_eq!(tool, "get_accounts");
            assert_eq!(actual, arguments);
        }
        _ => panic!("expected a remote tool call"),
    }
}

#[test]
fn runtime_metadata_does_not_relax_unknown_user_field_validation() {
    for key in ["token", "api_key", "endpoint", "agent_id", "unexpected"] {
        let mut input = enriched("list_tools");
        input[key] = json!("fixture-value");
        assert!(parse(input).err().unwrap().to_string().contains(key));
    }
}
