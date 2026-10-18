//! Live-drive browserctl against public endpoints to validate
//! net-log, fetch, axios, and diagnose actions end-to-end.
//!
//! Run: cargo run --example browser_validate

use codetether_agent::tool::Tool;
use codetether_agent::tool::browserctl::BrowserCtlTool;
use serde_json::{Value, json};

async fn call(tool: &BrowserCtlTool, args: Value) -> Value {
    let label = args.get("action").cloned().unwrap_or(Value::Null);
    println!("\n>>> {label}");
    let res = tool.execute(args).await.expect("tool.execute");
    println!("success={} output:", res.success);
    let out = &res.output;
    let trimmed = if out.len() > 2000 { &out[..2000] } else { out };
    println!("{trimmed}");
    serde_json::from_str(out).unwrap_or_else(|_| Value::String(out.clone()))
}

#[tokio::main]
async fn main() {
    let tool = BrowserCtlTool::new();

    call(&tool, json!({"action":"health"})).await;
    call(&tool, json!({"action":"detect"})).await;

    call(
        &tool,
        json!({
            "action":"start",
            "headless": true,
            "executable_path":"/usr/bin/chromium-browser",
            "user_data_dir":"/tmp/codetether-browser-validate"
        }),
    )
    .await;

    // Page that triggers XHR + fetch + axios (from a CDN).
    let html = r#"data:text/html,<html><head>
<script src='https://cdn.jsdelivr.net/npm/axios@1.7.7/dist/axios.min.js'></script>
</head><body><h1 id='h'>net-validate</h1><script>
window.__APP__={};
window.addEventListener('load',async()=>{
  try{await fetch('https://httpbin.org/get?via=fetch');}catch(e){}
  try{const x=new XMLHttpRequest();x.open('GET','https://httpbin.org/get?via=xhr');x.send();}catch(e){}
  try{window.__APP__.api=axios.create({baseURL:'https://httpbin.org'});await window.__APP__.api.get('/get?via=axios');}catch(e){}
});
</script></body></html>"#;
    let encoded = html.replace(' ', "%20").replace('\n', "%0A");
    call(
        &tool,
        json!({"action":"goto","url":encoded,"wait_until":"load"}),
    )
    .await;

    // give async requests a moment
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let log = call(&tool, json!({"action":"network_log","limit":50})).await;
    let via_fetch = log.to_string().contains("via=fetch");
    let via_xhr = log.to_string().contains("via=xhr");
    let via_axios = log.to_string().contains("via=axios");

    call(&tool, json!({"action":"diagnose"})).await;

    let fetch_res = call(
        &tool,
        json!({
            "action":"fetch",
            "method":"GET",
            "url":"https://httpbin.org/headers",
        }),
    )
    .await;

    let axios_res = call(
        &tool,
        json!({
            "action":"axios",
            "method":"POST",
            "url":"/post",
            "axios_path":"window.__APP__.api",
            "json_body": {"hello":"world","n":42}
        }),
    )
    .await;

    let xhr_res = call(
        &tool,
        json!({
            "action":"xhr",
            "method":"PUT",
            "url":"https://httpbin.org/put",
            "headers":{"Content-Type":"application/json","X-Via":"xhr-replay"},
            "body":"{\"xhr\":true,\"n\":7}",
            "with_credentials": true
        }),
    )
    .await;

    call(&tool, json!({"action":"stop"})).await;

    println!("\n===== VERDICT =====");
    println!("net_log captured fetch? {via_fetch}");
    println!("net_log captured xhr?   {via_xhr}");
    println!("net_log captured axios? {via_axios}");
    let fetch_ok = fetch_res
        .get("status")
        .and_then(|v| v.as_i64())
        .map(|s| s == 200)
        .unwrap_or_else(|| fetch_res.to_string().contains("\"status\":200"));
    println!("fetch action → 200?     {fetch_ok}");
    let axios_ok = axios_res.to_string().contains("\"hello\":\"world\"")
        || axios_res.to_string().contains("hello");
    println!("axios POST echoed body? {axios_ok}");
    let xhr_ok = xhr_res
        .get("status")
        .and_then(|v| v.as_i64())
        .map(|s| s == 200)
        .unwrap_or(false)
        && xhr_res.to_string().contains("xhr-replay");
    println!("xhr PUT → 200 + header echoed? {xhr_ok}");
    println!("===================");

    let all = via_fetch && via_xhr && via_axios && fetch_ok && axios_ok && xhr_ok;
    if !all {
        std::process::exit(1);
    }
}
