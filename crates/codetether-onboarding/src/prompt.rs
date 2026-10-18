//! Line-oriented interactive onboarding prompts.
use anyhow::Result;
use std::io::{self, Write};
/// Print and flush a prompt, then read one trimmed line.
/// # Arguments
/// * `prompt` - Prompt shown on standard output.
/// # Returns
/// The entered line without surrounding whitespace.
/// # Errors
/// Returns standard-input or standard-output I/O errors.
/// # Examples
/// ```no_run
/// let answer = codetether_onboarding::prompt_line("Choice: ")?;
/// # Ok::<(), anyhow::Error>(())
/// ```

pub fn prompt_line(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
