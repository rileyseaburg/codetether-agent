//! OKR status render behavior.
use codetether_okr::Okr;
pub(super) fn print(okr: &Okr) -> () {
    println!("\n=== OKR: {} ===\n", okr.title);
    println!("Description: {}", okr.description);
    println!("Status: {:?}", okr.status);
    println!("Progress: {:.1}%\n", okr.progress() * 100.0);

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
                for ev in &outcome.evidence {
                    println!("      Evidence: {}", ev);
                }
            }
        }
    }
}
