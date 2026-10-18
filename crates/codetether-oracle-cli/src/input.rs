//! Oracle CLI input responsibilities.
use crate::OracleValidateArgs;
use anyhow::{Context, Result};
use std::io::Read;

pub(super) fn load_source(args: &OracleValidateArgs) -> Result<String> {
    match (&args.file, &args.content) {
        (Some(path), None) => std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read source file {}", path.display())),
        (None, Some(content)) if content == "-" => {
            let mut stdin_content = String::new();
            std::io::stdin().read_to_string(&mut stdin_content)?;
            Ok(stdin_content)
        }
        (None, Some(content)) => Ok(content.clone()),
        (Some(_), Some(_)) => {
            anyhow::bail!("Provide either --file or --content, not both");
        }
        (None, None) => {
            anyhow::bail!("Provide --file <path> or --content <text|->");
        }
    }
}

pub(super) fn load_payload(args: &OracleValidateArgs) -> Result<String> {
    match (&args.payload, &args.payload_file) {
        (Some(payload), None) => Ok(payload.clone()),
        (None, Some(path)) => std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read payload file {}", path.display())),
        (Some(_), Some(_)) => {
            anyhow::bail!("Provide either --payload or --payload-file, not both");
        }
        (None, None) => {
            anyhow::bail!("Provide --payload <json> or --payload-file <path>");
        }
    }
}
