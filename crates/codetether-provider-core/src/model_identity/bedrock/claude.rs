//! Resolve one family of Bedrock model aliases.

pub(super) fn resolve(model: &str) -> Option<&'static str> {
    Some(match model {
        m if m == "fable" || m.ends_with("claude-fable-5") => "us.anthropic.claude-fable-5",

        "claude-sonnet-4.6" | "claude-4.6-sonnet" | "claude-sonnet-4-6" => {
            "us.anthropic.claude-sonnet-4-6"
        }
        "claude-sonnet-4.5" | "claude-4.5-sonnet" => "us.anthropic.claude-sonnet-4-5-20250929-v1:0",
        "claude-sonnet-4" | "claude-4-sonnet" => "us.anthropic.claude-sonnet-4-20250514-v1:0",
        "claude-haiku-4.5" | "claude-4.5-haiku" => "us.anthropic.claude-haiku-4-5-20251001-v1:0",

        // Handle full IDs without version suffix (common user input)
        "us.anthropic.claude-sonnet-4-6" => "us.anthropic.claude-sonnet-4-6",
        "us.anthropic.claude-sonnet-4-5" => "us.anthropic.claude-sonnet-4-5-20250929-v1:0",
        "us.anthropic.claude-sonnet-4" => "us.anthropic.claude-sonnet-4-20250514-v1:0",
        "us.anthropic.claude-haiku-4-5" => "us.anthropic.claude-haiku-4-5-20251001-v1:0",

        "claude-3.7-sonnet" | "claude-sonnet-3.7" => "us.anthropic.claude-3-7-sonnet-20250219-v1:0",
        "claude-3.5-sonnet-v2" | "claude-sonnet-3.5-v2" => {
            "us.anthropic.claude-3-5-sonnet-20241022-v2:0"
        }
        "claude-3.5-haiku" | "claude-haiku-3.5" => "us.anthropic.claude-3-5-haiku-20241022-v1:0",
        "claude-3.5-sonnet" | "claude-sonnet-3.5" => "us.anthropic.claude-3-5-sonnet-20240620-v1:0",
        "claude-3-opus" | "claude-opus-3" => "us.anthropic.claude-3-opus-20240229-v1:0",
        "claude-3-haiku" | "claude-haiku-3" => "us.anthropic.claude-3-haiku-20240307-v1:0",
        "claude-3-sonnet" | "claude-sonnet-3" => "us.anthropic.claude-3-sonnet-20240229-v1:0",

        _ => return None,
    })
}
