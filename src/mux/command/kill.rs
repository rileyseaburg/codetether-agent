//! Close one mux session; its server exits when no sessions remain.

use anyhow::Result;

use crate::mux::client::MuxConnection;
use crate::mux::protocol::{ClientRequest, ServerResponse};
use crate::mux::registry::{MuxRecord, SessionTarget};

pub(super) async fn run(target: &str) -> Result<()> {
    let target = crate::mux::registry::load(target).await?;
    run_target(target).await
}

pub(super) async fn run_target(target: SessionTarget) -> Result<()> {
    let name = target.session.as_str();
    let connection = MuxConnection::connect_server(&target.record).await;
    let response = match connection {
        Ok(mut connection) => {
            let request = ClientRequest::CloseSession {
                name: name.to_string(),
            };
            match connection.request(request).await {
                Ok(response) => response,
                Err(error) => return force(&target.record, name, error).await,
            }
        }
        Err(error) => return force(&target.record, name, error).await,
    };
    match response {
        ServerResponse::Snapshot { .. } | ServerResponse::ShuttingDown => {
            println!("stopped mux session '{name}'");
            Ok(())
        }
        ServerResponse::Error { message } => anyhow::bail!("{message}"),
        _ => anyhow::bail!("mux server returned an invalid close response"),
    }
}

async fn force(record: &MuxRecord, name: &str, error: anyhow::Error) -> Result<()> {
    tracing::warn!(session = name, %error, "Graceful mux session close failed");
    super::terminate::run(record).await?;
    crate::mux::registry::remove_key(&record.key).await?;
    println!("force-stopped mux server hosting '{name}'");
    Ok(())
}
