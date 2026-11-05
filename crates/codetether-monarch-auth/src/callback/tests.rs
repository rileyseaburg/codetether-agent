//! Callback validation rejects ambiguity, incorrect state, host and issuer.
use super::{Callback, parse::parse};

#[test]
fn rejects_state_replay_ambiguity_host_and_issuer() {
    let request =
        |query: &str| format!("GET /callback?{query} HTTP/1.1\r\nHost: 127.0.0.1:4567\r\n\r\n");
    let parse_request = |query: &str| parse(&request(query), "127.0.0.1:4567", "expected");
    assert!(matches!(
        parse_request("state=expected&code=abc"),
        Ok(Callback::Code(_))
    ));
    for query in [
        "code=abc",
        "state=wrong&code=abc",
        "state=expected&state=expected&code=abc",
        "state=expected&code=abc&error=denied",
        "state=expected&code=abc&iss=https://evil.test",
        "state=expected&code=",
    ] {
        assert!(parse_request(query).is_err());
    }
    assert!(parse(&request("state=expected&code=abc"), "evil.test", "expected").is_err());
    assert!(matches!(
        parse_request("state=expected&error=denied"),
        Ok(Callback::Denied)
    ));
}
