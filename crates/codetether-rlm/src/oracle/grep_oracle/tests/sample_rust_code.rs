//! Regression for sample_rust_code.

pub(super) fn sample_rust_code() -> String {
    r#"
use anyhow::Result;

/// Process data
pub async fn process(input: &str) -> Result<String> {
    let data = parse(input)?;
    Ok(data)
}

async fn parse(input: &str) -> Result<String> {
    Ok(input.to_uppercase())
}

pub struct Config {
    pub debug: bool,
}

impl Config {
    pub fn new() -> Self {
        Self { debug: false }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_process() {
        assert!(true);
    }
}
"#
    .to_string()
}
