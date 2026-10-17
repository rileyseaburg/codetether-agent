//! Retry a rejected schema envelope using the original legacy payload.
use super::*;
pub(super) async fn upload_legacy(
    settings: &CrashReporterSettings,
    client: &reqwest::Client,
    report: &CrashReport,
    status: StatusCode,
) -> Result<bool> {
    tracing::info!(
        status = %status,
        report_id = %report.report_id,
        "Crash endpoint rejected schema envelope; trying legacy payload"
    );

    let legacy_response = build_upload_request(settings, client, report)
        .header("X-CodeTether-Payload", "legacy")
        .json(report)
        .send()
        .await
        .context("send legacy crash payload")?;

    if legacy_response.status().is_success() {
        return Ok(true);
    }

    tracing::warn!(
        status = %legacy_response.status(),
        report_id = %report.report_id,
        "Crash report upload failed for both schema and legacy payloads"
    );
    return Ok(false);
}
