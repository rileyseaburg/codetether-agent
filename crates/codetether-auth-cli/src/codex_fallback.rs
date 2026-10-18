//! Authentication codex_fallback responsibilities.

pub(super) fn is_expected_codex_id_token_exchange_fallback(error: &anyhow::Error) -> bool {
    let msg = error.to_string().to_ascii_lowercase();
    msg.contains("missing organization_id")
        || (msg.contains("invalid_subject_token") && msg.contains("organization"))
}
