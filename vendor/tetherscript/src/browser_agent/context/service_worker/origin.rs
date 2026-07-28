//! Origin and URL normalization for service-worker scopes.

pub(crate) fn service_worker_origin(input: &str) -> String {
    let Some((scheme, rest)) = input.split_once("://") else {
        return input.split('#').next().unwrap_or(input).to_string();
    };
    let authority = rest
        .split(['/', '?', '#'])
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase();
    format!("{}://{}", scheme.to_ascii_lowercase(), authority)
}

pub(crate) fn service_worker_url(origin: &str, value: &str) -> String {
    if value.contains("://") {
        return value.split('#').next().unwrap_or(value).to_string();
    }
    let value = value.split('#').next().unwrap_or(value);
    if value.starts_with('/') {
        format!("{origin}{value}")
    } else {
        format!("{origin}/{value}")
    }
}
