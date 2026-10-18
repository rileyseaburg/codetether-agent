//! Prompt the user for goals and persist them as an OKR.

use anyhow::Result;
use std::io::{self, Write};

use codetether_forage_reporting::forage_println;
use codetether_okr::OkrRepository;

use super::EmptyReason;

/// Read goal lines from the user until a blank line is entered.
pub(super) fn read_goals(reason: EmptyReason) -> Vec<String> {
    match reason {
        EmptyReason::NoOkrs => {
            forage_println!("\nForage found no objectives to work on yet.");
        }
        EmptyReason::NoActionableWork => {
            forage_println!("\nForage found objectives, but no remaining actionable work.");
        }
    }
    forage_println!("What are your goals? Enter one per line; blank line to finish.");
    let mut goals = Vec::new();
    let stdin = io::stdin();
    loop {
        print!("  goal> ");
        let _ = io::stdout().flush();
        let mut line = String::new();
        if stdin.read_line(&mut line).unwrap_or(0) == 0 {
            break;
        }
        let trimmed = line.trim().to_string();
        if trimmed.is_empty() {
            break;
        }
        goals.push(trimmed);
    }
    goals
}

/// Build and persist an OKR capturing the user's stated goals.
pub(super) async fn persist_goals(repo: &OkrRepository, goals: &[String]) -> Result<()> {
    codetether_forage_core::seed::persist_clarified_goals(repo, goals).await?;
    forage_println!("Saved {} goal(s) as an active OKR.", goals.len());
    Ok(())
}
