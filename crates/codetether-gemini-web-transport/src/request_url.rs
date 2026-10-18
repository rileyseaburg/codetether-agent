//! Query parameters for the Gemini StreamGenerate endpoint.
use crate::*;
pub(crate) fn request_url(
    endpoint: &str,
    tokens: &SessionTokens,
    reqid: &str,
) -> Result<reqwest::Url> {
    reqwest::Url::parse_with_params(
        endpoint,
        &[
            ("bl", tokens.bl.as_str()),
            ("f.sid", tokens.f_sid.as_str()),
            ("hl", "en"),
            ("pageId", "none"),
            ("_reqid", reqid),
            ("rt", "c"),
        ],
    )
    .context("Failed to build Gemini endpoint URL")
}
