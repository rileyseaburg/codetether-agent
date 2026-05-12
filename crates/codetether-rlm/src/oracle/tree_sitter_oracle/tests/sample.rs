pub(super) fn rust_code() -> String {
    r#"
use anyhow::Result;

pub struct Config {
    pub debug: bool,
    pub timeout: u64,
}

impl Config {
    pub fn new() -> Self {
        Self { debug: false, timeout: 30 }
    }

    pub fn with_debug(mut self) -> Self {
        self.debug = true;
        self
    }
}

pub async fn process(input: &str) -> Result<String> {
    let data = parse(input)?;
    Ok(data.to_uppercase())
}

fn parse(input: &str) -> Result<String> {
    if input.is_empty() {
        return Err(anyhow!("empty input"));
    }
    Ok(input.to_string())
}

#[allow(dead_code)]
enum Status {
    Active,
    Inactive,
    Pending,
}
"#
    .to_string()
}
