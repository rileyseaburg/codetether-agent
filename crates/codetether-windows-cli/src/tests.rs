//! Platform readiness behavior without providers or model initialization.
use super::*;

#[tokio::test]
#[cfg(not(windows))]
async fn windows_ocr_probe_does_not_claim_linux_readiness() {
    let error = run(WindowsArgs {
        command: WindowsCommand::OcrStatus {
            require_ready: true,
        },
    })
    .await
    .unwrap_err();
    assert!(error.to_string().contains("query failed"));
}
