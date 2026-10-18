//! Registration password length and confirmation checks.
use crate::password::rpassword_prompt;
use anyhow::Result;

pub(super) fn read() -> Result<String> {
    let password = rpassword_prompt("Password (min 8 chars): ")?;
    if password.trim().len() < 8 {
        anyhow::bail!("Password must be at least 8 characters");
    }
    let confirm = rpassword_prompt("Confirm password: ")?;
    if password != confirm {
        anyhow::bail!("Passwords do not match");
    }

    Ok(password)
}
