fn main() {
    let enabled_requested = std::env::var("CODETETHER_TOOL_ROUTER_ENABLED")
        .map(|v| matches!(v.as_str(), "1" | "true" | "yes"))
        .unwrap_or(false);

    let disabled = std::env::var("CODETETHER_FUNCTIONGEMMA_DISABLED")
        .map(|v| matches!(v.as_str(), "1" | "true" | "yes"))
        .unwrap_or(true);

    let enabled = enabled_requested && !disabled;
    println!("Enabled: {}", enabled);
}
