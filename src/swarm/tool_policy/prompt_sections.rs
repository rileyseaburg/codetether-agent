pub fn coordination_prompt(read_only: bool, _model: &str) -> String {
    if read_only {
        return String::new();
    }
    "SHARING RESULTS:\nUse swarm_share to publish and retrieve findings from peer swarm participants. Recursive delegation is unavailable.".to_string()
}

pub fn workflow_prompt(_read_only: bool, _prd_filename: &str) -> String {
    String::new()
}
