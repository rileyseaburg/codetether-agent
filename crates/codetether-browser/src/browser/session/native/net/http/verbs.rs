use crate::browser::{
    BrowserError, BrowserOutput,
    request::{AxiosRequest, FetchRequest, XhrRequest},
};

type Session = super::super::super::super::BrowserSession;

pub(super) async fn fetch(
    _session: &Session,
    request: FetchRequest,
) -> Result<BrowserOutput, BrowserError> {
    super::send(&request.method, &request.url, request.headers, request.body).await
}

pub(super) async fn xhr(
    _session: &Session,
    request: XhrRequest,
) -> Result<BrowserOutput, BrowserError> {
    super::send(&request.method, &request.url, request.headers, request.body).await
}

pub(super) async fn axios(
    _session: &Session,
    request: AxiosRequest,
) -> Result<BrowserOutput, BrowserError> {
    let body = request.body.map(|value| value.to_string());
    super::send(&request.method, &request.url, request.headers, body).await
}
