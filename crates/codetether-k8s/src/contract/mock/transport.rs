//! In-memory HTTP transport with captured requests.
use super::{State, data, response};
use http::{Request, Response};
use kube::{Client, client::Body};
use std::sync::{Arc, Mutex};
pub(super) fn client(state: Arc<Mutex<State>>) -> Client {
    let service = tower::service_fn({
        let state = state.clone();
        move |request: Request<Body>| {
            let state = state.clone();
            async move {
                let method = request.method().to_string();
                let path = request.uri().path().to_string();
                let bytes = request.into_body().collect_bytes().await.unwrap();
                let body = serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null);
                let (code, response) = {
                    let mut state = state.lock().unwrap();
                    state.requests.push(data::RequestRecord {
                        method: method.clone(),
                        path: path.clone(),
                        body: body.clone(),
                    });
                    response::respond(&mut state, &method, &path, body)
                };
                Ok::<_, std::convert::Infallible>(
                    Response::builder()
                        .status(code)
                        .header("content-type", "application/json")
                        .body(Body::from(serde_json::to_vec(&response).unwrap()))
                        .unwrap(),
                )
            }
        }
    });
    Client::new(service, "fixture")
}
