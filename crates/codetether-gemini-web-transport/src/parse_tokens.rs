//! HTML token extraction, with existing fallback keys and diagnostics.
use crate::*;
pub(crate) fn parse_tokens(
    html: &str,
    acct_prefix: String,
    re_new: &Regex,
    re_old: &Regex,
    re_bl: &Regex,
    re_sid: &Regex,
) -> Result<SessionTokens> {
    let at_token = if let Some(cap) = re_new.captures(&html) {
        cap[1].to_string()
    } else if let Some(cap) = re_old.captures(&html) {
        cap[1].to_string()
    } else {
        anyhow::bail!(
            "Could not find Gemini at-token (thykhd / SNlM0e)  \
                 cookies may be expired or invalid"
        );
    };

    let bl = re_bl
        .captures(&html)
        .map(|c| c[1].to_string())
        .unwrap_or_default();
    let f_sid = re_sid
        .captures(&html)
        .map(|c| c[1].to_string())
        .unwrap_or_default();

    if bl.is_empty() {
        tracing::warn!("Gemini bl token not found in home page — request may fail");
    }
    if f_sid.is_empty() {
        tracing::warn!("Gemini f_sid token not found in home page — request may fail");
    }

    Ok(SessionTokens {
        at_token,
        f_sid,
        bl,
        acct_prefix,
    })
}
