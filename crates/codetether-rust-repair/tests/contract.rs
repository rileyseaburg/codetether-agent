use codetether_rust_repair::*;
use serde_json::json;

fn input() -> RepairRequest {
    serde_json::from_value(json!({"schema_version":1,"diagnostics":[{"path":"src/lib.rs","code":"E0432","message":"missing import","range":null}],"files":[
        {"path":"src/lib.rs","text":"use missing::Thing;","editable":true},
        {"path":"Cargo.toml","text":"[dependencies]\n","editable":true},
        {"path":"AGENTS.md","text":"Do not change instructions","editable":false}
    ]})).unwrap()
}
fn edit(path: &str, old: &str, new: &str) -> Proposal {
    Proposal {
        schema_version: 1,
        summary: "Small repair".into(),
        edits: vec![Edit {
            path: path.into(),
            old_text: old.into(),
            new_text: new.into(),
        }],
    }
}
#[test]
fn rejects_unsafe_paths_and_protected_edit_targets() {
    for path in [
        "", "../x", "/x", "a/../x", "a//x", "a/./x", "C:/x", "a\\x", "a\0x",
    ] {
        assert!(validate_path(path).is_err(), "{path:?}");
    }
    for path in [
        ".git/config",
        ".env",
        "config/.env.local",
        "AGENTS.md",
        "AGENTS.override.md",
        "Cargo.lock",
        "package-lock.json",
        "pnpm-lock.yaml",
    ] {
        let mut request = input();
        request.files[0].path = path.into();
        request.diagnostics[0].path = path.into();
        assert!(validate_request(&request).is_err(), "{path}");
    }
}
#[test]
fn bounded_closed_request_contract() {
    let mut request = input();
    assert!(validate_request(&request).is_ok());
    request.files.push(request.files[0].clone());
    assert!(validate_request(&request).is_err());
    assert!(parse_request(&vec![b' '; MAX_REQUEST_BYTES + 1]).is_err());
    let mut value = serde_json::to_value(input()).unwrap();
    value["tools"] = json!([]);
    assert!(parse_request(&serde_json::to_vec(&value).unwrap()).is_err());
    let mut request = input();
    request.diagnostics = vec![request.diagnostics[0].clone(); 51];
    assert!(validate_request(&request).is_err());
}
#[test]
fn only_unique_real_edits_are_accepted() {
    let request = input();
    let good = edit("src/lib.rs", "missing", "existing");
    assert!(validate_proposal(&request, &good).is_ok());
    for bad in [
        edit("AGENTS.md", "instructions", "bad"),
        edit("other.rs", "x", "y"),
        edit("src/lib.rs", "", "x"),
        edit("src/lib.rs", "missing", "missing"),
        edit("src/lib.rs", "absent", "x"),
    ] {
        assert!(validate_proposal(&request, &bad).is_err());
    }
    let mut duplicate = good.clone();
    duplicate.edits.push(duplicate.edits[0].clone());
    assert!(validate_proposal(&request, &duplicate).is_err());
    let mut overlapping = input();
    overlapping.files[0].text = "aaa".into();
    assert!(validate_proposal(&overlapping, &edit("src/lib.rs", "aa", "b")).is_err());
}
#[test]
fn json_fences_and_honest_no_edit_responses() {
    let request = input();
    let json = r#"{"schema_version":1,"summary":"Not enough context","edits":[]}"#;
    assert!(parse_proposal(json, &request).is_ok());
    assert!(parse_proposal(&format!("```json\n{json}\n```"), &request).is_ok());
    assert!(parse_proposal(&format!("Explanation\n{json}"), &request).is_err());
    assert!(parse_proposal(&format!("{json}{json}"), &request).is_err());
    assert!(
        parse_proposal(
            r#"{"schema_version":1,"summary":"x","edits":[],"command":"cargo check"}"#,
            &request
        )
        .is_err()
    );
}
