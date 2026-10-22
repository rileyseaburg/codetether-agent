//! Compare exact-repeat guidance with periodic review on the recorded loop.
use anyhow::Result;
use codetether_agent::provider::Role;
use codetether_repeat_guard::ProgressLedger;
#[test]
#[ignore = "requires CODETETHER_REPLAY_SESSION; no model calls or tool execution"]
fn recorded_loop_cannot_evade_review_by_varying_inspections() -> Result<()> {
    let path = std::env::var("CODETETHER_REPLAY_SESSION")?;
    let (source, _, _) = super::replay_support::load(std::path::Path::new(&path))?;
    let mut previous = ProgressLedger::default();
    let mut repaired = ProgressLedger::default();
    let (mut old_nudges, mut reviews) = (0, 0);
    for (index, message) in source.messages.iter().enumerate() {
        let turn = source.offset + index;
        if !(4414..4790).contains(&turn) {
            continue;
        }
        for ledger in [&mut previous, &mut repaired] {
            if message.role == Role::User {
                ledger.human_input(message);
            }
            ledger.observe_message(message, turn);
        }
        old_nudges += usize::from(previous.take_nudge(15).is_some());
        reviews += usize::from(repaired.take_review(15).is_some());
        repaired = serde_json::from_value(serde_json::to_value(&repaired)?)?;
    }
    let report = serde_json::json!({"start":4414,"end":4790,"old_nudges":old_nudges,"reviews":reviews,"observations":repaired.observations,"novel_observations":repaired.novel_observations});
    eprintln!("{report}");
    if let Ok(path) = std::env::var("CODETETHER_REPLAY_REPORT") {
        std::fs::write(
            format!("{path}.reviews.json"),
            serde_json::to_string_pretty(&report)?,
        )?;
    }
    anyhow::ensure!(
        reviews > old_nudges && reviews > 0,
        "incident did not exercise novelty bypass"
    );
    Ok(())
}
