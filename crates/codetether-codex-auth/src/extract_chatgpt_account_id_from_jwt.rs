//! extract chatgpt account id from jwt.
use crate::*;
use base64::Engine as _;
impl CodexOAuth {
    /// Read a workspace identifier from recognized JWT claim shapes.
    ///
    /// # Arguments
    /// * `jwt` - Token whose claims are inspected.
    ///
    /// # Returns
    /// A recognized account identifier, if decoding succeeds.
    ///
    /// # Examples
    /// ```
    /// assert_eq!(codetether_codex_auth::CodexOAuth::extract_chatgpt_account_id_from_jwt("invalid"), None);
    /// ```
    pub fn extract_chatgpt_account_id_from_jwt(jwt: &str) -> Option<String> {
        let payload_b64 = jwt.split('.').nth(1)?;
        let payload = URL_SAFE_NO_PAD.decode(payload_b64).ok()?;
        let value: Value = serde_json::from_slice(&payload).ok()?;

        value
            .get("https://api.openai.com/auth")
            .and_then(Value::as_object)
            .and_then(|auth| auth.get("chatgpt_account_id"))
            .and_then(Value::as_str)
            .map(|s| s.to_string())
            .or_else(|| {
                value
                    .get("chatgpt_account_id")
                    .and_then(Value::as_str)
                    .map(|s| s.to_string())
            })
            .or_else(|| {
                value
                    .get("organizations")
                    .and_then(Value::as_array)
                    .and_then(|orgs| orgs.first())
                    .and_then(|org| org.get("id"))
                    .and_then(Value::as_str)
                    .map(|s| s.to_string())
            })
    }
}
