//! Read and record the existing CLI confirmation response.
use super::Prompt;
use codetether_okr::{ApprovalDecision, OkrRun};
use std::io::Write;
/// Display the objective and read a yes/no decision from standard input.
/// # Arguments
/// * `run` - Run on which denial is recorded.
/// * `prompt` - Draft display information.
/// # Returns
/// True for y/yes; false after recording and displaying a denial.
/// # Errors
/// Returns standard-input read or standard-output flush errors.
/// # Examples
/// ```no_run
/// # fn example(run: &mut codetether_okr::OkrRun, prompt: codetether_relay_okr::cli_approval::Prompt) -> std::io::Result<()> {
/// let approved = codetether_relay_okr::cli_approval::ask(run, prompt)?;
/// # Ok(())
/// # }
/// ```
pub fn ask(run: &mut OkrRun, prompt: Prompt<'_>) -> std::io::Result<bool> {
    super::render::render(&prompt);
    print!("Approve OKR and start relay? [y/n]: ");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let input = input.trim().to_lowercase();
    if input != "y" && input != "yes" {
        run.record_decision(ApprovalDecision::deny(run.id, "User denied via CLI"));
        println!("❌ OKR denied. Relay not started.");
        println!("Use /autochat --no-prd for tactical execution without OKR/PRD tracking.");
        return Ok(false);
    }

    println!("✅ OKR approved! Starting Ralph PRD execution...\n");

    Ok(true)
}
