//! Redirect traversal, limit handling, and disabled-feature fallback.
use super::AuthTrace;
use anyhow::Result;
#[cfg(feature = "tetherscript")]
pub fn walk(url: &str, max_redirects: u8) -> Result<AuthTrace> {
    use reqwest::blocking::Client;
    use reqwest::redirect::Policy;
    use tetherscript::browser_session::BrowserSession;

    use super::cookie::serialize_cookie;
    use super::url::resolve;

    let client = Client::builder().redirect(Policy::none()).build()?;
    let mut session = BrowserSession::new();
    let mut steps = Vec::new();
    let mut current = url.to_string();
    let mut truncated = false;
    let limit = usize::from(max_redirects);
    for hop in 0..=limit {
        let step = super::request::fetch(&client, &mut session, &current)?;
        let status = step.status;
        let location = step.location.clone();
        steps.push(step);
        match (status, location) {
            (300..=399, Some(loc)) if hop < limit => current = resolve(&current, &loc),
            (300..=399, Some(_)) => {
                truncated = true;
                break;
            }
            _ => break,
        }
    }
    let cookies_after = session.cookies.iter().map(serialize_cookie).collect();
    Ok(AuthTrace {
        final_url: current,
        redirect_count: steps.len().saturating_sub(1),
        truncated,
        steps,
        cookies_after,
    })
}

#[cfg(not(feature = "tetherscript"))]
pub fn walk(_: &str, _: u8) -> Result<AuthTrace> {
    anyhow::bail!("auth-trace requires the `tetherscript` feature");
}
