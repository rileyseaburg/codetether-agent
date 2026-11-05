//! Construct the resource-bound authorization URL with PKCE S256.
use crate::{
    callback,
    discovery::{AUTHORIZE, RESOURCE},
};
use anyhow::{Result, anyhow};
use url::Url;

pub(super) fn authorization_url(
    client_id: &str,
    redirect: &str,
    scope: &str,
    state: &str,
    verifier: &str,
) -> Result<Url> {
    let mut url = Url::parse(AUTHORIZE).map_err(|_| anyhow!("Invalid authorization endpoint"))?;
    url.query_pairs_mut().extend_pairs([
        ("response_type", "code"),
        ("client_id", client_id),
        ("redirect_uri", redirect),
        ("scope", scope),
        ("state", state),
        ("resource", RESOURCE),
        ("code_challenge_method", "S256"),
        ("code_challenge", &callback::challenge(verifier)),
    ]);
    Ok(url)
}
