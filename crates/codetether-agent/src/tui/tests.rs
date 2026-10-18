//! Legacy regression cases awaiting mapping to current APIs.
//! The other 32 cases now belong to their implementation crates.
#[cfg(test)]
mod tests {
    use super::{
        estimate_cost, is_secure_environment_from_values, normalize_for_convergence,
        view_mode_help_rows,
    };

    #[test]
    fn view_mode_help_rows_include_every_view_mode() {
        let rows = view_mode_help_rows();
        assert_eq!(rows.len(), 9);
        assert!(rows.iter().any(|row| row.contains("Chat")));
        assert!(rows.iter().any(|row| row.contains("Swarm")));
        assert!(rows.iter().any(|row| row.contains("Ralph")));
        assert!(rows.iter().any(|row| row.contains("Bus Log")));
        assert!(rows.iter().any(|row| row.contains("Protocol")));
        assert!(rows.iter().any(|row| row.contains("Session Picker")));
        assert!(rows.iter().any(|row| row.contains("Model Picker")));
        assert!(rows.iter().any(|row| row.contains("Agent Picker")));
        assert!(rows.iter().any(|row| row.contains("File Picker")));
    }

    #[test]
    fn normalize_for_convergence_ignores_case_and_punctuation() {
        let a = normalize_for_convergence("Done! Next Step: Add tests.");
        let b = normalize_for_convergence("done next step add tests");
        assert_eq!(a, b);
    }

    #[test]
    fn secure_environment_detection_respects_explicit_flags() {
        assert!(is_secure_environment_from_values(Some(true), None, None));
        assert!(!is_secure_environment_from_values(
            Some(false),
            Some(true),
            Some("secure")
        ));
    }

    #[test]
    fn secure_environment_detection_uses_environment_name_fallback() {
        assert!(is_secure_environment_from_values(None, None, Some("PROD")));
        assert!(is_secure_environment_from_values(
            None,
            None,
            Some("production")
        ));
        assert!(!is_secure_environment_from_values(None, None, Some("dev")));
    }

    #[test]
    fn minimax_m25_pricing_estimate_matches_rates() {
        let cost = estimate_cost("minimax/MiniMax-M2.5", 1_000_000, 1_000_000)
            .expect("MiniMax M2.5 cost should be available");
        assert!((cost - 1.5).abs() < 1e-9); // $0.3 + $1.2 = $1.5
    }

    #[test]
    fn minimax_m25_highspeed_pricing() {
        let cost = estimate_cost("MiniMax-M2.5-highspeed", 1_000_000, 1_000_000)
            .expect("MiniMax M2.5 Highspeed cost should be available");
        assert!((cost - 3.0).abs() < 1e-9); // $0.6 + $2.4 = $3.0
    }
}
