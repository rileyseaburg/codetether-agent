//! OKR report run text behavior.
use codetether_okr::OkrRun;
pub(super) fn print(run: &OkrRun) -> () {
    println!("\n=== OKR Run Report: {} ===\n", run.name);
    println!("OKR ID: {}", run.okr_id);
    println!("Status: {:?}", run.status);

    if let Some(corr) = &run.correlation_id {
        println!("Correlation ID: {}", corr);
    }
    if let Some(session) = &run.session_id {
        println!("Session ID: {}", session);
    }
    if let Some(checkpoint) = &run.relay_checkpoint_id {
        println!("Relay Checkpoint ID: {}", checkpoint);
    }
    println!("  Started: {}", run.started_at);
    if let Some(completed) = run.completed_at {
        println!("Completed: {}", completed);
    }

    if !run.kr_progress.is_empty() {
        println!("\nKR Progress:");
        for (kr_id, progress) in &run.kr_progress {
            println!("  {}: {:.1}%", kr_id, progress * 100.0);
        }
    }
}
