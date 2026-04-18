use super::{BrowserSession, access};
use crate::browser::{BrowserError, BrowserOutput, output::EvalOutput, request::EvalRequest};
use std::time::Duration;

const EVAL_TIMEOUT: Duration = Duration::from_secs(30);

pub(super) async fn run(
    session: &BrowserSession,
    request: EvalRequest,
) -> Result<BrowserOutput, BrowserError> {
    let page = access::current_page(session).await?;
    let script = script(request.expression)?;
    let result = tokio::time::timeout(EVAL_TIMEOUT, page.evaluate_expression(script))
        .await
        .map_err(|_| BrowserError::EvaluationTimeout)??;
    let body = result.object().value.clone().ok_or_else(|| {
        BrowserError::OperationFailed("evaluation wrapper returned no value".into())
    })?;
    Ok(BrowserOutput::Eval(EvalOutput {
        result: value(body)?,
    }))
}

fn value(body: serde_json::Value) -> Result<serde_json::Value, BrowserError> {
    match body
        .get("__codetether_kind")
        .and_then(serde_json::Value::as_str)
    {
        Some("value") => body.get("value").cloned().ok_or_else(|| {
            BrowserError::OperationFailed("evaluation wrapper returned no result".into())
        }),
        Some("error") => Err(BrowserError::EvalNonSerializable {
            type_: string(&body, "type", "unknown"),
            subtype: None,
            description: string(&body, "description", "value cannot be serialized to JSON"),
        }),
        _ => Err(BrowserError::OperationFailed(
            "evaluation wrapper returned unexpected payload".into(),
        )),
    }
}

fn string(body: &serde_json::Value, key: &str, fallback: &str) -> String {
    body.get(key)
        .and_then(serde_json::Value::as_str)
        .unwrap_or(fallback)
        .to_string()
}

fn script(expression: String) -> Result<String, BrowserError> {
    let source = serde_json::to_string(&expression)?;
    Ok(format!(
        r#"(() => {{
  const __ct_serialize = (__ct_value) => {{
    const __ct_name = __ct_value && typeof __ct_value === "object" && __ct_value.constructor && __ct_value.constructor.name ? __ct_value.constructor.name : typeof __ct_value;
    if (__ct_value === undefined || typeof __ct_value === "function" || typeof __ct_value === "symbol" || typeof __ct_value === "bigint") return {{ "__codetether_kind": "error", "type": typeof __ct_value, "description": __ct_name }};
    if (typeof Node !== "undefined" && __ct_value instanceof Node) return {{ "__codetether_kind": "error", "type": "object", "description": __ct_name }};
    if (__ct_value instanceof Date || __ct_value instanceof RegExp || __ct_value instanceof Map || __ct_value instanceof Set || __ct_value instanceof WeakMap || __ct_value instanceof WeakSet || __ct_value instanceof ArrayBuffer || ArrayBuffer.isView(__ct_value) || __ct_value instanceof Error || __ct_value instanceof Promise) return {{ "__codetether_kind": "error", "type": typeof __ct_value, "description": __ct_name }};
    try {{
      const __ct_json = JSON.stringify(__ct_value);
      if (__ct_json === undefined) return {{ "__codetether_kind": "error", "type": typeof __ct_value, "description": __ct_name }};
      return {{ "__codetether_kind": "value", "value": JSON.parse(__ct_json) }};
    }} catch (__ct_error) {{
      return {{ "__codetether_kind": "error", "type": typeof __ct_value, "description": __ct_error?.message || String(__ct_error) }};
    }}
  }};
  const __ct_value = eval({source});
  return __ct_value && typeof __ct_value.then === "function" ? __ct_value.then(__ct_serialize) : __ct_serialize(__ct_value);
}})()"#
    ))
}
