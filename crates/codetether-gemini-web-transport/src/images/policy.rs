use anyhow::{Result, anyhow, ensure};
use reqwest::Url;

pub(super) fn image_url(value: &str) -> Result<Url> {
    ensure!(value.len() <= 16384, "image_url_invalid");
    let url = Url::parse(value).map_err(|_| anyhow!("image_url_invalid"))?;
    let allowed = match url.host_str() {
        Some("lh3.googleusercontent.com") => ["/gg-dl/", "/rd-gg-dl/"]
            .iter()
            .any(|p| url.path().starts_with(p)),
        Some("work.fife.usercontent.google.com") => url.path().starts_with("/rd-gg-dl/"),
        _ => false,
    };
    ensure!(
        allowed
            && url.scheme() == "https"
            && url.port().is_none()
            && url.username().is_empty()
            && url.password().is_none()
            && url.fragment().is_none(),
        "image_url_disallowed"
    );
    Ok(url)
}
pub(super) fn rendition(value: &str) -> Result<Url> {
    let mut url = image_url(value)?;
    ensure!(
        url.host_str() == Some("lh3.googleusercontent.com") && url.path().starts_with("/gg-dl/"),
        "image_reference_disallowed"
    );
    let path = url.path().to_string();
    let base = path
        .rsplit_once("=s")
        .filter(|(_, size)| {
            let number = size.strip_suffix("-rj").unwrap_or(size);
            !number.is_empty() && number.bytes().all(|c| c.is_ascii_digit())
        })
        .map_or(path.as_str(), |(base, _)| base);
    url.set_path(&format!("{base}=s1024-rj"));
    let pairs: Vec<_> = url
        .query_pairs()
        .filter(|(k, _)| k != "alr")
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();
    url.set_query(None);
    url.query_pairs_mut()
        .extend_pairs(pairs)
        .append_pair("alr", "yes");
    Ok(url)
}
