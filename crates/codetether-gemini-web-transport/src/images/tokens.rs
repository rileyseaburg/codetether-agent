use anyhow::{Result, ensure};
use regex::Regex;

pub(super) struct Tokens {
    pub at: String,
    pub build: String,
    pub sid: String,
    pub prefix: String,
}
fn field(html: &str, key: &str) -> String {
    let pattern = format!(r#""{key}"\s*:\s*("(?:[^"\\]|\\.)*")"#);
    let value = Regex::new(&pattern).ok().and_then(|re| {
        re.captures(html)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_owned())
    });
    value
        .and_then(|v| serde_json::from_str(&v).ok())
        .unwrap_or_default()
}
pub(super) fn parse(html: &str, path: &str) -> Result<Tokens> {
    let at = field(html, "SNlM0e");
    ensure!(!at.is_empty(), "image_session_not_authenticated");
    let build = field(html, "cfb2h");
    let sid = field(html, "FdrFJe");
    ensure!(
        !build.is_empty() && !sid.is_empty(),
        "image_session_metadata_missing"
    );
    let parts: Vec<_> = path.split('/').collect();
    let prefix = if parts.len() >= 4
        && parts[1] == "u"
        && !parts[2].is_empty()
        && parts[2].bytes().all(|b| b.is_ascii_digit())
    {
        format!("/u/{}", parts[2])
    } else {
        String::new()
    };
    Ok(Tokens {
        at,
        build,
        sid,
        prefix,
    })
}
