//! Optional-argument or interactive email input.
use anyhow::Result;
use std::io::{self, Write};

pub(super) fn read(email: Option<String>) -> Result<String> {
    let email = match email {
        Some(e) => e,
        None => {
            print!("Email: ");
            io::stdout().flush()?;
            let mut email = String::new();
            io::stdin().read_line(&mut email)?;
            email.trim().to_string()
        }
    };

    if email.is_empty() {
        anyhow::bail!("Email is required");
    }

    Ok(email)
}
