//! Crash reporting prompt yes no.
use super::*;
pub(super) fn prompt_yes_no(prompt: &str) -> io::Result<bool> {
    let mut stdout = io::stdout();

    loop {
        write!(stdout, "{prompt}")?;
        stdout.flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let normalized = input.trim().to_ascii_lowercase();
        if normalized.is_empty() {
            return Ok(false);
        }
        if matches!(normalized.as_str(), "y" | "yes") {
            return Ok(true);
        }
        if matches!(normalized.as_str(), "n" | "no") {
            return Ok(false);
        }

        writeln!(stdout, "Please answer 'y' or 'n'.")?;
    }
}
