//! OKR report okr keys behavior.
use codetether_okr::Okr;
pub(super) fn print(okr: &Okr, evidence: bool) -> () {
    println!("Key Results:");
    for kr in &okr.key_results {
        let kr_progress = kr.progress() * 100.0;
        println!("\n  [{}] {}", kr.id, kr.title);
        println!(
            "  Progress: {:.1}/{} {} ({:.1}%)",
            kr.current_value, kr.target_value, kr.unit, kr_progress
        );
        println!("  Status: {:?}", kr.status);

        if !kr.outcomes.is_empty() {
            println!("  Outcomes:");
            for outcome in &kr.outcomes {
                println!("    - {} ({:?})", outcome.description, outcome.outcome_type);
                if let Some(value) = outcome.value {
                    println!("      Value: {}", value);
                }
                if evidence {
                    for ev in &outcome.evidence {
                        println!("      Evidence: {}", ev);
                    }
                }
            }
        } else if evidence {
            println!("  (No outcomes recorded)");
        }
    }
}
