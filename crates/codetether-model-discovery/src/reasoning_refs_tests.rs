use super::expand;

#[test]
fn sol_includes_ultra_while_luna_stops_at_max() {
    let sol = expand("openai-codex", "openai-codex/gpt-5.6-sol".into());
    let luna = expand("openai-codex", "openai-codex/gpt-5.6-luna".into());
    assert!(sol.contains(&"openai-codex/gpt-5.6-sol".into()));
    assert!(sol.contains(&"openai-codex/gpt-5.6-sol:ultra".into()));
    assert!(sol.contains(&"openai-codex/gpt-5.6-sol-fast:ultra".into()));
    assert!(!luna.iter().any(|model| model.ends_with(":ultra")));
    assert!(luna.iter().any(|model| model.ends_with(":max")));
}

#[test]
fn other_providers_are_unchanged() {
    let model = "openrouter/openai/gpt-5.6-sol".to_string();
    assert_eq!(expand("openrouter", model.clone()), vec![model]);
}

#[test]
fn astra_picker_exposes_catalog_levels_for_normal_and_fast() {
    let variants = expand("openai-codex", "openai-codex/gpt-6-astra".into());
    for suffix in ["", "-fast", "-ultrafast"] {
        for level in ["low", "medium", "high", "xhigh", "max", "ultra"] {
            assert!(variants.contains(&format!("openai-codex/gpt-6-astra{suffix}:{level}")));
        }
    }
    assert_eq!(variants.len(), 21);
}

#[test]
fn sol61_exposes_all_efforts_and_fast_variants() {
    let models = expand("openai-codex", "openai-codex/gpt-6.1-sol".into());
    for tier in ["", "-fast"] {
        for effort in ["low", "medium", "high", "xhigh", "max", "ultra"] {
            assert!(models.contains(&format!("openai-codex/gpt-6.1-sol{tier}:{effort}")));
        }
    }
    assert_eq!(models.len(), 14);
}
