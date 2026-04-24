use anyhow::{Context, Result, bail};
use std::time::Duration;

pub async fn check(
    url: &str,
    method: &str,
    expect_status: u16,
    expect_body_contains: &[String],
    timeout_secs: u64,
) -> Result<()> {
    let method = method
        .parse::<reqwest::Method>()
        .with_context(|| format!("invalid HTTP method `{method}`"))?;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()?;
    let response = client.request(method, url).send().await?;
    let status = response.status().as_u16();
    let body = response.text().await?;

    if status != expect_status {
        bail!("URL `{url}` returned {status}, expected {expect_status}");
    }
    for expected in expect_body_contains {
        if !body.contains(expected) {
            bail!("URL `{url}` body did not contain `{expected}`");
        }
    }
    Ok(())
}
