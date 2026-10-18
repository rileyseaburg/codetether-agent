//! Cross-platform hidden token input; never accept secrets in process arguments.

use anyhow::{Result, ensure};
use crossterm::event;
use std::io::{IsTerminal, Write};

pub(super) fn read(stdin: bool) -> Result<String> {
    if stdin {
        return super::token_stdin::read();
    }
    ensure!(
        std::io::stdin().is_terminal(),
        "A terminal is required; use --stdin for a secure pipe"
    );
    let _restore = super::terminal_guard::enter()?;
    eprint!("Vault token (hidden): ");
    std::io::stderr().flush()?;
    let mut value = String::new();
    loop {
        if super::token_event::apply(event::read()?, &mut value)? {
            break;
        }
        ensure!(value.len() <= 16384, "Vault token input is too large");
    }
    eprint!("\r\n");
    Ok(value.trim().to_owned())
}
