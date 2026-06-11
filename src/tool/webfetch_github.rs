//! GitHub request auth helpers for webfetch.

use super::super::bash_github::GitHubCommandAuth;
use reqwest::{RequestBuilder, Url, header};

pub(super) async fn auth_for(url: &Url, cwd: Option<&str>) -> Option<GitHubCommandAuth> {
    match super::super::bash_github::load_github_url_auth(url, cwd).await {
        Ok(auth) => auth,
        Err(error) => {
            tracing::warn!(%url, error = %error, "Failed to load GitHub auth for webfetch");
            None
        }
    }
}

pub(super) fn apply(request: RequestBuilder, auth: Option<&GitHubCommandAuth>) -> RequestBuilder {
    let Some(token) = auth.and_then(github_token) else {
        return request;
    };
    request
        .bearer_auth(token)
        .header(header::ACCEPT, "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
}

pub(super) fn redact(mut body: String, auth: Option<&GitHubCommandAuth>) -> String {
    let Some(auth) = auth else {
        return body;
    };
    for secret in &auth.redactions {
        body = body.replace(secret, "[REDACTED]");
    }
    body
}

fn github_token(auth: &GitHubCommandAuth) -> Option<&str> {
    auth.env
        .iter()
        .find(|(key, _)| *key == "GITHUB_TOKEN")
        .map(|(_, value)| value.as_str())
}
