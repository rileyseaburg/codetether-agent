//! OAuth callback wait_any handling.
use crate::wait::wait_for_oauth_callback;
use anyhow::Result;
use tokio::net::TcpListener;
use tokio::time::Duration;

pub(super) async fn wait_for_oauth_callback_any(
    mut listeners: Vec<TcpListener>,
    timeout: Duration,
) -> Result<(String, String)> {
    match listeners.len() {
        0 => anyhow::bail!("No OAuth callback listeners were available"),
        1 => {
            let listener = listeners.pop().expect("length checked");
            wait_for_oauth_callback(listener, timeout).await
        }
        _ => {
            let listener2 = listeners.pop().expect("length checked");
            let listener1 = listeners.pop().expect("length checked");

            let mut waiter1 = Box::pin(wait_for_oauth_callback(listener1, timeout));
            let mut waiter2 = Box::pin(wait_for_oauth_callback(listener2, timeout));

            tokio::select! {
                result1 = &mut waiter1 => {
                    match result1 {
                        Ok(callback) => Ok(callback),
                        Err(err1) => match waiter2.await {
                            Ok(callback) => Ok(callback),
                            Err(err2) => anyhow::bail!("{}; {}", err1, err2),
                        },
                    }
                }
                result2 = &mut waiter2 => {
                    match result2 {
                        Ok(callback) => Ok(callback),
                        Err(err2) => match waiter1.await {
                            Ok(callback) => Ok(callback),
                            Err(err1) => anyhow::bail!("{}; {}", err2, err1),
                        },
                    }
                }
            }
        }
    }
}
