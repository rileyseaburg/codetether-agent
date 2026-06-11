use super::super::{
    PlannedRelayProfile, PlannedRelayResponse, build_runtime_profile_from_plan,
    extract_json_payload,
};

#[test]
fn extract_json_payload_parses_markdown_wrapped_json() {
    let wrapped = "Here is the plan:\n```json\n{\"profiles\":[{\"name\":\"auto-db\",\"specialty\":\"database\",\"mission\":\"Own schema and queries\",\"capabilities\":[\"sql\",\"indexing\"]}]}\n```";
    let parsed: PlannedRelayResponse =
        extract_json_payload(wrapped).expect("should parse wrapped JSON");
    assert_eq!(parsed.profiles.len(), 1);
    assert_eq!(parsed.profiles[0].name, "auto-db");
}

#[test]
fn build_runtime_profile_normalizes_and_deduplicates_name() {
    let planned = PlannedRelayProfile {
        name: "Data Specialist".to_string(),
        specialty: "data engineering".to_string(),
        mission: "Prepare datasets for downstream coding".to_string(),
        capabilities: vec!["ETL".to_string(), "sql".to_string()],
    };

    let profile = build_runtime_profile_from_plan(planned, &["auto-data-specialist".to_string()])
        .expect("profile should be built");

    assert_eq!(profile.name, "auto-data-specialist-2");
    assert!(profile.capabilities.iter().any(|cap| cap == "relay"));
    assert!(profile.capabilities.iter().any(|cap| cap == "autochat"));
}
