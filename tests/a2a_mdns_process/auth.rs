use serde_json::json;

pub async fn assert_boundary(endpoint: &str, foreign_control: &str, own_control: &str) {
    let client = reqwest::Client::new();
    let body = json!({
        "jsonrpc": "2.0", "id": 1, "method": "message/send",
        "params": {"message": {
            "messageId": uuid::Uuid::new_v4(), "role": "user",
            "parts": [{"kind": "text", "text":
                "Hello from auth proof (http://local). I am online and available for A2A collaboration."}]
        }}
    });
    let missing = client.post(endpoint).json(&body).send().await.unwrap();
    assert_eq!(missing.status(), 401, "missing capability must be rejected");
    let foreign = client
        .post(endpoint)
        .bearer_auth(foreign_control)
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(
        foreign.status(),
        401,
        "another peer's control token must fail"
    );
    let own = client
        .post(endpoint)
        .bearer_auth(own_control)
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(own.status(), 200, "the peer's control token must pass");
    let response: serde_json::Value = own.json().await.unwrap();
    assert_eq!(response["result"]["status"]["state"], "completed");
    let card = client
        .get(format!("{endpoint}/.well-known/agent.json"))
        .send()
        .await
        .unwrap();
    assert_eq!(card.status(), 200, "agent card must remain discoverable");
}
