//! OKR list render behavior.
use codetether_okr::Okr;
pub(super) fn print(okrs: &[Okr], evidence: bool) -> () {
    println!("\n=== OKRs ===\n");
    for okr in okrs {
        let progress = okr.progress() * 100.0;
        println!("[{}] {} - {:.1}% complete", okr.id, okr.title, progress);
        println!("  Status: {:?}", okr.status);
        if let Some(owner) = &okr.owner {
            println!("  Owner: {}", owner);
        }
        for kr in &okr.key_results {
            let kr_progress = kr.progress() * 100.0;
            println!(
                "  - {}: {:.1}/{} {} ({:.1}%)",
                kr.title, kr.current_value, kr.target_value, kr.unit, kr_progress
            );
            if evidence && !kr.outcomes.is_empty() {
                for outcome in &kr.outcomes {
                    println!("    Evidence: {}", outcome.description);
                    for ev in &outcome.evidence {
                        println!("      - {}", ev);
                    }
                }
            }
        }
        println!();
    }
}
