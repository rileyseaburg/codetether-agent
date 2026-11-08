//! RFC7636 compatibility and fresh-state generation checks.
use super::{challenge, random_secret};

#[test]
fn pkce_matches_rfc7636_vector() {
    assert_eq!(
        challenge("dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk"),
        "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM"
    );
    let a = random_secret().unwrap();
    assert_eq!(a.len(), 43);
    assert_ne!(a, random_secret().unwrap());
}
