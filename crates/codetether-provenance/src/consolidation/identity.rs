//! Provenance identity contract.

#[test]
fn identity_precedence_snapshot() {
    let values:Vec<_>=[None,Some(""),Some(" User:ABC/123 "),Some("---"),Some("雪"),Some("a_b.c")]
        .iter().map(|id|serde_json::json!({"name":crate::git_author_name(*id),"email":crate::git_author_email(*id,Some("worker"),Some("run"))})).collect();
    println!("IDENTITIES={}", serde_json::to_string(&values).unwrap());
}
