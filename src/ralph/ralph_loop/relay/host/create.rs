//! Construct a native session with the selected role model and system message.
use super::*;
pub(super) async fn run(
    model: String,
    name: &str,
    instructions: &str,
    relay_bus: &Arc<AgentBus>,
) -> anyhow::Result<Session> {
    let mut session = Session::new().await?;
    session.metadata.model = Some(model);
    session.set_agent_name(name.to_string());
    session.bus = Some(relay_bus.clone());
    session.add_message(Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: instructions.to_string(),
        }],
    });
    Ok(session)
}
