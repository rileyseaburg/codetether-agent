use super::query::ListedAgent;
use crate::session::Session;
use crate::tool::agent::store;

pub(super) async fn entry(name: &str, owner: &str) -> store::AgentEntry {
    store::AgentEntry {
        name: name.into(),
        instructions: "test".into(),
        session: Session::new().await.unwrap(),
        parent: None,
        owner_session_id: Some(owner.into()),
        depth: 1,
        model_id: None,
    }
}

pub(super) fn names(agents: &[ListedAgent]) -> Vec<&str> {
    agents
        .iter()
        .map(|agent| agent.agent_name.as_str())
        .collect()
}
