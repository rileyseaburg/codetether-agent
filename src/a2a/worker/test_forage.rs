use serde_json::json;

#[test]
fn metadata_lookup_reads_nested_forage_keys() {
    let metadata = json!({"forage": {"execute": true, "top": 5}})
        .as_object()
        .cloned()
        .unwrap();
    assert_eq!(super::metadata_bool(&metadata, &["execute"]), Some(true));
    assert_eq!(super::metadata_usize(&metadata, &["top"]), Some(5));
}

#[test]
fn build_forage_args_defaults_prompt_to_moonshot_and_local_mode() {
    let args = super::build_forage_args(
        "Expand enterprise adoption in regulated markets",
        "forage task",
        &serde_json::Map::new(),
        Some("openrouter/z-ai/glm-5".to_string()),
    );
    assert_eq!(
        args.moonshots,
        vec!["Expand enterprise adoption in regulated markets".to_string()]
    );
    assert!(args.no_s3);
    assert_eq!(args.model.as_deref(), Some("openrouter/z-ai/glm-5"));
    assert_eq!(args.execution_engine, "run");
}

#[test]
fn build_forage_args_honors_nested_forage_configuration() {
    let metadata = json!({"forage": {"top": 7, "loop": true, "max_cycles": 2, "execute": true, "no_s3": false, "moonshots": ["Ship autonomous OKR execution", "Reduce operator toil"], "moonshot_required": true, "moonshot_min_alignment": "0.25", "execution_engine": "swarm", "run_timeout_secs": 1200, "fail_fast": true, "swarm_strategy": "stage", "swarm_max_subagents": 4, "swarm_max_steps": 42, "swarm_subagent_timeout_secs": 180 }}).as_object().cloned().unwrap();
    let args = super::build_forage_args("fallback prompt", "title", &metadata, None);
    assert_eq!(args.top, 7);
    assert!(args.loop_mode);
    assert_eq!(args.max_cycles, 2);
    assert!(args.execute);
    assert!(!args.no_s3);
    assert_eq!(args.moonshots.len(), 2);
    assert!(args.moonshot_required);
    assert!((args.moonshot_min_alignment - 0.25).abs() < f64::EPSILON);
    assert_eq!(args.execution_engine, "swarm");
    assert_eq!(args.run_timeout_secs, 1200);
    assert!(args.fail_fast);
    assert_eq!(args.swarm_strategy, "stage");
    assert_eq!(args.swarm_max_subagents, 4);
    assert_eq!(args.swarm_max_steps, 42);
    assert_eq!(args.swarm_subagent_timeout_secs, 180);
}
