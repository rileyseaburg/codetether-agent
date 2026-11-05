//! State and construction for one short-lived Monarch MCP session.
use anyhow::Result;
mod close;
mod initialize;
mod post;
mod rpc;

pub(crate) struct Client {
    http: reqwest::Client,
    endpoint: String,
    token: String,
    session: Option<String>,
    version: String,
    next_id: u64,
}

impl Client {
    pub(crate) fn new(endpoint: String, token: String) -> Result<Self> {
        Ok(Self {
            http: reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .timeout(std::time::Duration::from_secs(60))
                .build()?,
            endpoint,
            token,
            session: None,
            version: "2025-03-26".into(),
            next_id: 0,
        })
    }
}
