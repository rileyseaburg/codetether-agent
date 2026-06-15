use anyhow::{Context, Result};
use reqwest::{Client, RequestBuilder};
use serde_json::{Value, json};

pub const BATCHEXECUTE_PATH: &str = "/_/BardChatUi/data/batchexecute";

pub struct GeminiWebTransportRequest<'a> {
    pub origin: &'a str,
    pub cookie_header: &'a str,
    pub bl: &'a str,
    pub f_sid: &'a str,
    pub at_token: &'a str,
    pub rpcid: &'a str,
    pub prompt: &'a str,
    pub model_id: &'a str,
    pub reqid: &'a str,
}

pub fn batched_rpc_payload(rpcid: &str, prompt: &str, model_id: &str) -> String {
    let inner = json!([[[prompt, 0]], [], model_id]);
    let outer = json!([[[rpcid, inner.to_string(), null, "generic"]]]);
    outer.to_string()
}

pub fn batchexecute_request(
    client: &Client,
    input: GeminiWebTransportRequest<'_>,
) -> Result<RequestBuilder> {
    let endpoint = format!("{}{}", input.origin, BATCHEXECUTE_PATH);
    let url = reqwest::Url::parse_with_params(
        &endpoint,
        &[
            ("rpcids", input.rpcid),
            ("source-path", "/app"),
            ("f.sid", input.f_sid),
            ("bl", input.bl),
            ("hl", "en"),
            ("_reqid", input.reqid),
            ("rt", "c"),
        ],
    )
    .context("Failed to build Gemini batchexecute URL")?;
    let payload = batched_rpc_payload(input.rpcid, input.prompt, input.model_id);
    Ok(client
        .post(url)
        .header("Cookie", input.cookie_header)
        .header("X-Same-Domain", "1")
        .header("Origin", input.origin)
        .header("Referer", format!("{}/app", input.origin))
        .header("Content-Type", "application/x-www-form-urlencoded;charset=UTF-8")
        .form(&[("f.req", payload), ("at", input.at_token.to_string())]))
}
