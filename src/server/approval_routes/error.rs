use axum::http::StatusCode;

pub(super) type RouteError = (StatusCode, String);
pub(super) type RouteResult<T> = Result<axum::Json<T>, RouteError>;

pub(super) fn bad(message: impl Into<String>) -> RouteError {
    (StatusCode::BAD_REQUEST, message.into())
}

pub(super) fn map(error: anyhow::Error) -> RouteError {
    let message = error.to_string();
    if message.contains("not found") {
        return (StatusCode::NOT_FOUND, message);
    }
    if message.contains("already decided") {
        return (StatusCode::CONFLICT, message);
    }
    (StatusCode::BAD_REQUEST, message)
}
