use crate::support::{body, candidate, frame, inner};
use codetether_gemini_web_protocol::images::parse_response;
use serde_json::json;

#[test]
fn explicit_errors_and_invalid_payloads_do_not_leak_signed_urls() {
    let secret = "https://images.test/private?signature=never-log";
    for bad in [
        json!(["er", secret]),
        json!(["wrb.fr", null, 5]),
        json!(["wrb.fr", null, secret]),
    ] {
        let valid = frame(inner(vec![candidate(&[secret], "rc_test")]));
        let error = parse_response(&body(vec![valid, bad])).unwrap_err();
        assert!(!format!("{error:#}").contains(secret));
    }
}

#[test]
fn refusal_and_unavailable_text_are_fixed_errors() {
    for (text, error) in [
        ("Sorry, something went wrong.", "generation_answer_error"),
        (
            "I can't create it right now",
            "generation_image_creation_unavailable",
        ),
        (
            "I can't create any for you at the moment.",
            "generation_image_creation_unavailable",
        ),
    ] {
        let mut choice = candidate(&[], "rc_test");
        choice[1] = json!([text]);
        let result = parse_response(&body(vec![frame(inner(vec![choice]))]));
        assert_eq!(result.unwrap_err().to_string(), error);
    }
}

#[test]
fn caps_images_but_returns_raw_untrusted_strings_for_transport_policy() {
    let urls: Vec<String> = (0..9).map(|n| format!("https://images.test/{n}")).collect();
    let refs: Vec<&str> = urls.iter().map(String::as_str).collect();
    let result = parse_response(&body(vec![frame(inner(vec![candidate(&refs, "rc_test")]))]));
    assert_eq!(result.unwrap_err().to_string(), "generation_image_limit");
    let urls = [
        "http://127.0.0.1/raw?signed=value",
        "https://images.test/a=s2048",
    ];
    let result =
        parse_response(&body(vec![frame(inner(vec![candidate(&urls, "rc_test")]))])).unwrap();
    assert_eq!(result.urls, urls);
}
