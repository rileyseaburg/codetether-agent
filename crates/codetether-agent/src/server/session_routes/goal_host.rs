//! Native goal controller binding for shared session HTTP routes.
use crate::session::Session;
use crate::session::tasks::{GoalEdit, control};
use codetether_session_http::GoalHost;
#[async_trait::async_trait]
impl GoalHost for Session {
    async fn read_goal(id: &str) -> Result<serde_json::Value, control::GoalControlError> {
        control::read(id).await
    }
    async fn update_goal(
        id: &str,
        edit: GoalEdit,
    ) -> Result<serde_json::Value, control::GoalControlError> {
        control::update_user(id, edit).await
    }
}
