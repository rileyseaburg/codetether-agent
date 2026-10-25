use crate::images::{ImageClient, client::State, cookies::now, http, tokens::Tokens};
use anyhow::{Result, anyhow, ensure};
use codetether_gemini_web_protocol::images::{self, ImageResponse};
use reqwest::Url;

impl ImageClient {
    pub(in crate::images) async fn generate_request(
        &self,
        state: &State,
        tokens: Tokens,
        payload: String,
    ) -> Result<ImageResponse> {
        let endpoint = format!(
            "{}{}/_/BardChatUi/data/assistant.lamda.BardFrontendService/StreamGenerate",
            crate::images::ORIGIN,
            tokens.prefix
        );
        let mut url = Url::parse(&endpoint).map_err(|_| anyhow!("image_request_url_invalid"))?;
        url.query_pairs_mut()
            .extend_pairs([
                ("bl", tokens.build.as_str()),
                ("f.sid", tokens.sid.as_str()),
                ("hl", "en"),
                ("rt", "c"),
            ])
            .append_pair("_reqid", &(now() % 900000 + 100000).to_string());
        let request = self
            .http
            .post(url.clone())
            .header("Accept", "application/json")
            .header("Origin", crate::images::ORIGIN)
            .header("Referer", "https://gemini.google.com/app")
            .header("X-Same-Domain", "1")
            .header(
                "x-goog-ext-525001261-jspb",
                images::model_header(&tokens.build),
            )
            .form(&[("f.req", payload.as_str()), ("at", tokens.at.as_str())]);
        let request = http::cookie(request, state.base.header(&url, &state.updates))?;
        let response = http::send(request).await?;
        ensure!(
            response.status().as_u16() == 200,
            "image_generation_http_error"
        );
        let bytes = http::body(response, crate::images::LIMIT).await?;
        images::parse_response(&bytes).map_err(crate::images::protocol_error::classify)
    }
}
