//! Minimal Vault responses; fixture tokens are never used outside localhost.
use serde_json::{Value, json};

pub(super) fn lookup(renewable: bool, ttl: u64) -> Value {
    json!({"lease_id":"", "lease_duration":0, "renewable":false,
        "request_id":"fixture", "data": {
        "accessor":"fixture", "creation_time":0, "creation_ttl":ttl,
        "display_name":"fixture", "entity_id":"", "expire_time":null,
        "explicit_max_ttl":0, "id":"fixture-token-not-a-secret", "identity_policies":[],
        "issue_time":null, "meta":null, "num_uses":0, "orphan":false,
        "path":"auth/token/create", "policies":["default"], "renewable":renewable, "ttl":ttl
    }})
}

#[test]
fn lookup_fixture_matches_the_vault_sdk_envelope() {
    let response: vaultrs::api::EndpointResult<
        vaultrs::api::token::responses::LookupTokenResponse,
    > = serde_json::from_value(lookup(false, 10)).unwrap();
    assert_eq!(response.data.unwrap().renewable, Some(false));
}

pub(super) fn renewal(renewable: bool, ttl: u64) -> Value {
    json!({"auth": {
        "client_token":"fixture-token-not-a-secret", "accessor":"fixture",
        "policies":["default"], "token_policies":["default"], "metadata":null,
        "lease_duration":ttl, "renewable":renewable, "entity_id":"", "token_type":"service"
    }})
}
