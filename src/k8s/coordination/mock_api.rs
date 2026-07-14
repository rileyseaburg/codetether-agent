//! In-memory Kubernetes API service for coordination tests.

use http::{Request, Response, StatusCode};
use kube::{Client, client::Body};
use std::convert::Infallible;
use std::sync::{Arc, Mutex};

pub type Requests = Arc<Mutex<Vec<Request<Body>>>>;

#[derive(Clone)]
pub struct Expected {
    pub method: http::Method,
    pub status: StatusCode,
    pub body: serde_json::Value,
}

pub fn client(expected: Vec<Expected>) -> (Client, Arc<Mutex<Vec<Request<Body>>>>) {
    let queue = Arc::new(Mutex::new(expected));
    let requests = Arc::new(Mutex::new(Vec::new()));
    let service = tower::service_fn({
        let queue = Arc::clone(&queue);
        let requests = Arc::clone(&requests);
        move |request: Request<Body>| {
            let expected = queue.lock().unwrap().remove(0);
            assert_eq!(request.method(), expected.method);
            requests.lock().unwrap().push(request);
            async move {
                let bytes = serde_json::to_vec(&expected.body).unwrap();
                Ok::<_, Infallible>(
                    Response::builder()
                        .status(expected.status)
                        .header("content-type", "application/json")
                        .body(Body::from(bytes))
                        .unwrap(),
                )
            }
        }
    });
    (Client::new(service, "default"), requests)
}

pub fn lease(name: &str, holder: &str, renewed: &str, version: &str) -> serde_json::Value {
    serde_json::json!({
        "apiVersion":"coordination.k8s.io/v1", "kind":"Lease",
        "metadata":{"name":name,"namespace":"default","resourceVersion":version,"uid":"uid-1"},
        "spec":{"holderIdentity":holder,"leaseDurationSeconds":30,"renewTime":renewed}
    })
}
