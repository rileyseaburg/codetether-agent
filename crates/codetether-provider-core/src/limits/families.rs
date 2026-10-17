//! Remaining family limits after the more specific model checks.
pub(super) fn remaining(m: &str) -> usize {
    if m.contains("minimax-m3") || m.contains("minimaxm3") || m.contains("minimax/m3") {
        1_000_000
    } else if m.contains("minimax") || m.contains("m2.5") {
        256_000
    } else if m.contains("qwen") || m.contains("qwq") {
        131_072
    } else if m.contains("deepseek-v4") {
        1_048_576
    } else if m.contains("deepseek-r1")
        || m.contains("deepseek-v3")
        || m.contains("deepseek-chat")
        || m.contains("deepseek-reasoner")
    {
        128_000
    } else if m.contains("llama-4") || m.contains("llama4") {
        256_000
    } else if m.contains("nova-pro") || m.contains("nova-lite") || m.contains("nova-premier") {
        300_000
    } else if m.contains("nova-micro") || m.contains("nova-2") {
        128_000
    } else if m.contains("moonshot") || m.contains("k1.5") || m.contains("k1.6") {
        200_000
    } else if m.contains("mistral-large") || m.contains("magistral") {
        128_000
    } else if m.contains("mistral") {
        32_000
    } else if m.contains("jamba") {
        256_000
    } else {
        128_000 // conservative default
    }
}
