//! Upload a schema envelope and delegate eligible legacy-payload retries.
use super::*;
pub(super) async fn upload_report(
    settings: &CrashReporterSettings,
    client: &reqwest::Client,
    report: &CrashReport,
) -> Result<bool> {
    let envelope = CrashEnvelope {
        schema: CRASH_SCHEMA,
        source: CRASH_SOURCE,
        sent_at: Utc::now(),
        install_id: &settings.install_id,
        report,
    };

    let response = build_upload_request(settings, client, report)
        .json(&envelope)
        .send()
        .await
        .context("send schema crash payload")?;

    let status = response.status();
    if status.is_success() {
        return Ok(true);
    }

    if should_retry_with_legacy_payload(status) {
        return upload_legacy(settings, client, report, status).await;
    }

    tracing::warn!(
        status = %status,
        report_id = %report.report_id,
        "Crash report upload rejected"
    );
    Ok(false)
}
