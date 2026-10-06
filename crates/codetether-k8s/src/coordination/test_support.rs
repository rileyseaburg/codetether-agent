use super::mock_api;
use super::repository::LeaseRepository;
use super::resource::ResourceKey;
use http::{Method, StatusCode};

pub fn key(kind: &'static str) -> ResourceKey {
    ResourceKey::new("default", kind, "target")
}

pub fn repo(expected: Vec<mock_api::Expected>) -> (LeaseRepository, mock_api::Requests) {
    let (client, requests) = mock_api::client(expected);
    (LeaseRepository::new(client, "default"), requests)
}

pub fn expected(method: Method, status: StatusCode, body: serde_json::Value) -> mock_api::Expected {
    mock_api::Expected {
        method,
        status,
        body,
    }
}

pub fn conflict() -> serde_json::Value {
    serde_json::json!({
        "apiVersion":"v1","kind":"Status","status":"Failure",
        "message":"already exists","reason":"AlreadyExists","code":409
    })
}
