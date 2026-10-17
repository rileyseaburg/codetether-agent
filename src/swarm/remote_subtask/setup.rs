use super::*;
pub(super) fn payload(args: SwarmSubagentArgs) -> Result<RemoteSubtaskPayload> {
    let payload_b64 = if let Some(payload) = args.payload_base64 {
        payload
    } else {
        std::env::var(&args.payload_env)
            .with_context(|| format!("Missing swarm payload env var {}", args.payload_env))?
    };
    let payload = decode_payload(&payload_b64)?;

    Ok(payload)
}
pub(super) fn workspace(payload: &RemoteSubtaskPayload) -> Result<PathBuf> {
    let working_dir = payload
        .working_dir
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    std::env::set_current_dir(&working_dir)
        .with_context(|| format!("Failed to set working dir to {}", working_dir.display()))?;

    Ok(working_dir)
}
