//! Resolve one family of Bedrock model aliases.

pub(super) fn resolve(model: &str) -> Option<&'static str> {
    Some(match model {
        // --- Amazon Nova ---
        "nova-pro" => "amazon.nova-pro-v1:0",
        "nova-lite" => "amazon.nova-lite-v1:0",
        "nova-micro" => "amazon.nova-micro-v1:0",
        "nova-premier" => "us.amazon.nova-premier-v1:0",

        // --- Meta Llama ---
        "llama-4-maverick" | "llama4-maverick" => "us.meta.llama4-maverick-17b-instruct-v1:0",
        "llama-4-scout" | "llama4-scout" => "us.meta.llama4-scout-17b-instruct-v1:0",
        "llama-3.3-70b" | "llama3.3-70b" => "us.meta.llama3-3-70b-instruct-v1:0",
        "llama-3.2-90b" | "llama3.2-90b" => "us.meta.llama3-2-90b-instruct-v1:0",
        "llama-3.2-11b" | "llama3.2-11b" => "us.meta.llama3-2-11b-instruct-v1:0",
        "llama-3.2-3b" | "llama3.2-3b" => "us.meta.llama3-2-3b-instruct-v1:0",
        "llama-3.2-1b" | "llama3.2-1b" => "us.meta.llama3-2-1b-instruct-v1:0",
        "llama-3.1-70b" | "llama3.1-70b" => "us.meta.llama3-1-70b-instruct-v1:0",
        "llama-3.1-8b" | "llama3.1-8b" => "us.meta.llama3-1-8b-instruct-v1:0",
        "llama-3-70b" | "llama3-70b" => "meta.llama3-70b-instruct-v1:0",
        "llama-3-8b" | "llama3-8b" => "meta.llama3-8b-instruct-v1:0",

        // --- Mistral ---
        "mistral-large-3" | "mistral-large" => "mistral.mistral-large-3-675b-instruct",
        "mistral-large-2402" => "mistral.mistral-large-2402-v1:0",
        "mistral-small" => "mistral.mistral-small-2402-v1:0",
        "mixtral-8x7b" => "mistral.mixtral-8x7b-instruct-v0:1",
        "pixtral-large" => "us.mistral.pixtral-large-2502-v1:0",
        "magistral-small" => "mistral.magistral-small-2509",

        _ => return None,
    })
}
