pub(super) fn worker_public_url_for_port(public_url: Option<&str>, port: u16) -> Option<String> {
    let configured = public_url
        .map(str::trim)
        .filter(|value| !value.is_empty())?;
    reqwest::Url::parse(configured)
        .map(|mut url| {
            let _ = url.set_port(Some(port));
            url.as_str().trim_end_matches('/').to_string()
        })
        .ok()
        .or_else(|| Some(configured.trim_end_matches('/').to_string()))
}
