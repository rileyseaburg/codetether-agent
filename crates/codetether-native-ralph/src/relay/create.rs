//! Construct a native session with the selected role model and system message.
use super::*;
pub(super) async fn run<S: Session>(
    model: String,
    name: &str,
    instructions: &str,
    relay_bus: &Arc<AgentBus>,
) -> anyhow::Result<S> {
    let mut session = S::new().await?;
    session.configure(
        model,
        name.to_string(),
        relay_bus.clone(),
        Message {
            role: Role::System,
            content: vec![ContentPart::Text {
                text: instructions.to_string(),
            }],
        },
    );
    Ok(session)
}
