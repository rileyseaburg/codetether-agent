pub fn join_error(join_error: tokio::task::JoinError) -> anyhow::Error {
    if join_error.is_panic() {
        anyhow::anyhow!("Kiln plugin task panicked: {join_error}")
    } else {
        anyhow::anyhow!("Kiln plugin task join failed: {join_error}")
    }
}
