impl OpenAiCodexProvider {
    fn parse_responses_usage(usage: Option<&Value>) -> Usage {
        let Some(usage) = usage else {
            return Usage::default();
        };

        let prompt_tokens = usage
            .get("prompt_tokens")
            .or_else(|| usage.get("input_tokens"))
            .and_then(Value::as_u64)
            .unwrap_or(0) as usize;

        let completion_tokens = usage
            .get("completion_tokens")
            .or_else(|| usage.get("output_tokens"))
            .and_then(Value::as_u64)
            .unwrap_or(0) as usize;

        let total_tokens = usage
            .get("total_tokens")
            .and_then(Value::as_u64)
            .unwrap_or((prompt_tokens + completion_tokens) as u64)
            as usize;

        // OpenAI Responses API exposes the cache hit count under
        // `prompt_tokens_details.cached_tokens` (chat/completions compat)
        // or the newer `input_tokens_details.cached_tokens`. Either way
        // it is a *subset* of `prompt_tokens`, so we subtract it to
        // avoid double-counting when the cost estimator later applies
        // the cache-read discount.
        let cached_tokens = usage
            .get("prompt_tokens_details")
            .or_else(|| usage.get("input_tokens_details"))
            .and_then(|d| d.get("cached_tokens"))
            .and_then(Value::as_u64)
            .unwrap_or(0) as usize;

        Usage {
            prompt_tokens: prompt_tokens.saturating_sub(cached_tokens),
            completion_tokens,
            total_tokens,
            cache_read_tokens: if cached_tokens > 0 {
                Some(cached_tokens)
            } else {
                None
            },
            cache_write_tokens: None,
        }
    }
}
