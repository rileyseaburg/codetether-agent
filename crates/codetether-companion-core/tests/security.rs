use codetether_companion_core::{Error, OwnerCredential, require_origin};
use serde::Deserialize;

#[derive(Deserialize)]
struct Authorization {
    header: Option<String>,
    status: u16,
}
#[derive(Deserialize)]
struct Origin {
    origin: Option<String>,
    status: u16,
}
#[derive(Deserialize)]
struct Fixtures {
    owner: String,
    authorization: Vec<Authorization>,
    allowed_origin: String,
    origins: Vec<Origin>,
}
fn status(result: Result<(), Error>) -> u16 {
    result.map_or_else(|e| e.status(), |()| 200)
}
#[test]
fn shared_authentication_and_origin_cases() {
    let fixtures: Fixtures =
        serde_json::from_str(include_str!("../fixtures/security.json")).unwrap();
    let owner = OwnerCredential::new(&fixtures.owner).unwrap();
    for case in fixtures.authorization {
        assert_eq!(status(owner.authorize(case.header.as_deref())), case.status);
    }
    for case in fixtures.origins {
        let result = require_origin(case.origin.as_deref(), &fixtures.allowed_origin);
        assert_eq!(status(result), case.status);
    }
    assert!(OwnerCredential::new("short").is_err());
}
#[test]
fn error_statuses_are_explicit() {
    assert_eq!(Error::Input.status(), 400);
    assert_eq!(Error::NotFound.status(), 404);
    assert_eq!(Error::Ended.status(), 410);
}
