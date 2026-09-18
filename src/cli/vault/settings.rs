//! Read-only status and explicit local URL/logout operations.

use crate::secrets::login;
use anyhow::Result;

pub(super) fn url(address: &str) -> Result<()> {
    let address = login::normalize(address)?;
    let mut profile =
        login::load()?.unwrap_or_else(|| login::Profile::without_token(address.clone()));
    if profile.address != address {
        profile = login::Profile::without_token(address);
    }
    login::save(&profile)?;
    println!(
        "Vault URL saved. Credentials for a different URL were discarded; run codetether vault login."
    );
    Ok(())
}

pub(super) fn logout() -> Result<()> {
    let mut profile = login::current()?;
    profile.token = None;
    login::save(&profile)?;
    println!(
        "Saved Vault credential removed. External tokens were not revoked; existing processes are unchanged."
    );
    Ok(())
}
