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
    let body = result
        .object()
        .value
        .clone()
        .ok_or_else(|| wrapper_error("evaluation wrapper returned no value"))?;
    Ok(BrowserOutput::Eval(EvalOutput {
        result: value(body)?,
    }))
}

fn value(body: serde_json::Value) -> Result<serde_json::Value, BrowserError> {
    match body
        .get("__codetether_kind")
        .and_then(serde_json::Value::as_str)
    {
        Some("value") => body
            .get("value")
            .cloned()
            .ok_or_else(|| wrapper_error("evaluation wrapper returned no result")),
        Some("error") => Err(BrowserError::EvalNonSerializable {
            type_: string(&body, "type", "unknown"),
            subtype: None,
            description: string(&body, "description", "value cannot be serialized to JSON"),
        }),
        _ => Err(wrapper_error(
            "evaluation wrapper returned unexpected payload",
        )),
    }
}

fn string(body: &serde_json::Value, key: &str, fallback: &str) -> String {
    body.get(key)
        .and_then(serde_json::Value::as_str)
        .unwrap_or(fallback)
        .to_string()
}

fn wrapper_error(message: &str) -> BrowserError {
    BrowserError::EvalWrapperError(message.to_string())
}

fn script(expression: String) -> Result<String, BrowserError> {
    let source = serde_json::to_string(&expression)?;
    Ok(format!(
        r#"(() => {{
  const __ct_kind = "__codetether_kind";
  const __ct_name = (__ct_value) => __ct_value && typeof __ct_value === "object" && __ct_value.constructor && typeof __ct_value.constructor.name === "string" ? __ct_value.constructor.name : typeof __ct_value;
  const __ct_fail = (__ct_value, __ct_description) => ({{
    [__ct_kind]: "error",
    type: typeof __ct_value,
    description: __ct_description || __ct_name(__ct_value),
  }});
  const __ct_is_node = (__ct_value) => typeof __ct_value === "object" && __ct_value !== null && typeof __ct_value.nodeType === "number" && typeof __ct_value.nodeName === "string";
  const __ct_walk = (__ct_value, __ct_seen) => {{
    if (__ct_value === null || typeof __ct_value === "string" || typeof __ct_value === "boolean") return __ct_value;
    if (typeof __ct_value === "number") {{
      if (Number.isFinite(__ct_value)) return __ct_value;
      throw __ct_fail(__ct_value, "non-finite number: " + String(__ct_value));
    }}
    if (__ct_value === undefined || typeof __ct_value === "function" || typeof __ct_value === "symbol" || typeof __ct_value === "bigint") throw __ct_fail(__ct_value, __ct_name(__ct_value));
    if (__ct_is_node(__ct_value)) throw __ct_fail(__ct_value, __ct_name(__ct_value));
    if (typeof __ct_value !== "object") throw __ct_fail(__ct_value, __ct_name(__ct_value));
    if (__ct_seen.has(__ct_value)) throw __ct_fail(__ct_value, "circular reference");
    const __ct_tag = Object.prototype.toString.call(__ct_value);
    if (__ct_tag !== "[object Object]" && __ct_tag !== "[object Array]") throw __ct_fail(__ct_value, __ct_name(__ct_value));
    if (__ct_tag === "[object Object]" && __ct_value.constructor && __ct_value.constructor.name && __ct_value.constructor.name !== "Object") throw __ct_fail(__ct_value, __ct_name(__ct_value));
    __ct_seen.add(__ct_value);
    try {{
      if (Array.isArray(__ct_value)) return __ct_value.map((__ct_item) => __ct_walk(__ct_item, __ct_seen));
      const __ct_out = {{}};
      for (const [__ct_key, __ct_item] of Object.entries(__ct_value)) {{
        __ct_out[__ct_key] = __ct_walk(__ct_item, __ct_seen);
      }}
      return __ct_out;
    }} finally {{
      __ct_seen.delete(__ct_value);
    }}
  }};
  const __ct_serialize = (__ct_value) => {{
    try {{
      return {{ [__ct_kind]: "value", value: __ct_walk(__ct_value, new WeakSet()) }};
    }} catch (__ct_error) {{
      if (__ct_error && __ct_error[__ct_kind] === "error") return __ct_error;
      return __ct_fail(__ct_value, __ct_error?.message || String(__ct_error));
    }}
  }};
  const __ct_async_function = Object.getPrototypeOf(async function () {{}}).constructor;
  const __ct_value = __ct_async_function("return (" + {source} + ");")();
  return __ct_value && typeof __ct_value.then === "function" ? __ct_value.then(__ct_serialize) : __ct_serialize(__ct_value);
}})()"#
    ))
}
