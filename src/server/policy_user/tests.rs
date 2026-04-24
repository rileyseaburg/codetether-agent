use super::*;

#[test]
fn claims_drive_policy_user() {
    let claims = JwtClaims {
        subject: Some("user-1".into()),
        roles: vec!["viewer".into()],
        scopes: vec!["agent:read".into()],
        tenant_id: Some("tenant-1".into()),
        auth_source: Some("api_key".into()),
        topics: vec![],
    };
    let user = from_claims(&claims);
    assert_eq!(user.roles, vec!["viewer".to_string()]);
    assert_eq!(user.auth_source, "api_key");
}
