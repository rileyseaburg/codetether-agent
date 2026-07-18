//! Background dispatch of one claimed mailbox item.

use super::{claim, params, receipt};

pub(crate) fn next(agent_id: String) {
    tokio::spawn(async move {
        let item = match claim::next(&agent_id).await {
            Ok(Some(item)) => item,
            Ok(None) => return,
            Err(error) => {
                tracing::warn!(agent_id, %error, "Mailbox claim failed");
                return;
            }
        };
        let Some(guard) = super::super::super::execution_state::try_start(&agent_id) else {
            let _ = receipt::release(&agent_id, &item.id).await;
            return;
        };
        let input = params::queued(&item);
        if let Err(error) = super::super::super::message::run::execute(
            agent_id.clone(),
            item.message,
            item.images,
            &input,
            guard,
            Some(item.id.clone()),
        )
        .await
        {
            let _ = receipt::release(&agent_id, &item.id).await;
            tracing::warn!(agent_id, %error, "Queued follow-up failed");
        }
    });
}
