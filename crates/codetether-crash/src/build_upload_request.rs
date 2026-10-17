//! Crash reporting build upload request.
use super::*;
pub(super) fn build_upload_request(
    settings: &CrashReporterSettings,
    client: &reqwest::Client,
    report: &CrashReport,
) -> reqwest::RequestBuilder {
    let mut request = client
        .post(&settings.endpoint)
        .header("X-CodeTether-Schema", CRASH_SCHEMA)
        .header("X-CodeTether-Source", CRASH_SOURCE)
        .header("X-CodeTether-Install-Id", &settings.install_id)
        .header("X-CodeTether-Report-Id", &report.report_id)
        .header("X-CodeTether-App-Version", &settings.app_version);

    if let Some(token) = &settings.auth_token {
        request = request.bearer_auth(token);
    }
    if let Some(api_key) = &settings.api_key {
        request = request.header("X-CodeTether-API-Key", api_key);
    }

    request
}
