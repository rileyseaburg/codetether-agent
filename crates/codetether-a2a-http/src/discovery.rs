//! Retrieve the peer agent card.
use super::*;
impl A2AClient {
    /// Get the agent card
    pub async fn get_agent_card(&self) -> Result<AgentCard> {
        let url = format!("{}/.well-known/agent.json", self.base_url);
        let mut req = self.client.get(&url);

        if let Some(ref auth) = self.auth
            && let Some(ref creds) = auth.credentials
        {
            req = req.bearer_auth(creds);
        }

        let res = req.send().await?.error_for_status()?;
        let card: AgentCard = res.json().await?;
        Ok(card)
    }
}
