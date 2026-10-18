//! Human-readable cookie import completion report.
pub(super) fn print(
    count: usize,
    provider_id: &str,
    dropped_expired: usize,
    dropped_non_auth: usize,
    earliest_expiry: Option<i64>,
    can_read_back: bool,
) {
    println!(
        "Saved {} cookies to HashiCorp Vault provider '{}'.",
        count, provider_id
    );
    println!(
        "Dropped {} expired and {} non-auth cookies.",
        dropped_expired, dropped_non_auth
    );
    if let Some(epoch) = earliest_expiry
        && let Some(ts) = chrono::DateTime::from_timestamp(epoch, 0)
    {
        println!(
            "Earliest cookie expiry: {} (rotate at least 24h before this).",
            ts.to_rfc3339()
        );
    }
    println!(
        "Vault path: {}/{}",
        std::env::var("VAULT_SECRETS_PATH").unwrap_or_else(|_| "codetether/providers".to_string()),
        provider_id
    );
    println!(
        "Read-back verification: {}",
        if can_read_back { "ok" } else { "failed" }
    );
}
