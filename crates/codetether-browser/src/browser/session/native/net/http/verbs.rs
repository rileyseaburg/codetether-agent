use crate::browser::{
    BrowserError, BrowserOutput,
    request::{AxiosRequest, FetchRequest, XhrRequest},
};

type Session = super::super::super::super::BrowserSession;

pub(in crate::browser::session::native) async fn fetch(
    _session: &Session,
    request: FetchRequest,
) -> Result<BrowserOutput, BrowserError> {
    super::send(&request.method, &request.url, request.headers, request.body).await
}

pub(in crate::browser::session::native) async fn xhr(
    _session: &Session,
    request: XhrRequest,
) -> Result<BrowserOutput, BrowserError> {
    super::send(&request.method, &request.url, request.headers, request.body).await
}

pub(in crate::browser::session::native) async fn axios(
    _session: &Session,
    request: AxiosRequest,
) -> Result<BrowserOutput, BrowserError> {
    let body = request.body.map(|value| value.to_string());
    super::send(&request.method, &request.url, request.headers, body).await
}
