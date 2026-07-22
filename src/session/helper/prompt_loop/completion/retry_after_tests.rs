use super::seconds;

#[test]
fn extracts_and_bounds_server_delay() {
    assert_eq!(
        seconds(&anyhow::anyhow!("busy [retry_after_seconds=12]")),
        Some(12)
    );
    assert_eq!(
        seconds(&anyhow::anyhow!("busy [retry_after_seconds=999]")),
        Some(60)
    );
    assert_eq!(seconds(&anyhow::anyhow!("busy")), None);
}
