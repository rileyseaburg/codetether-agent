use super::*;

#[test]
fn payload_roundtrip() {
    let payload = RemoteSubtaskPayload {
        swarm_id: "swarm-1".to_string(),
        subtask_id: "subtask-1".to_string(),
        subtask_name: "name".to_string(),
        specialty: "specialist".to_string(),
        instruction: "do work".to_string(),
        context: "ctx".to_string(),
        provider: "zai".to_string(),
        model: "glm-5".to_string(),
        max_steps: 20,
        timeout_secs: 180,
        working_dir: Some("/workspace".to_string()),
        read_only: true,
        verification: false,
        probe_interval_secs: 5,
    };
    let encoded = encode_payload(&payload).expect("encode");
    let decoded = decode_payload(&encoded).expect("decode");
    assert_eq!(decoded.subtask_id, "subtask-1");
    assert_eq!(decoded.provider, "zai");
    assert!(decoded.read_only);
}
