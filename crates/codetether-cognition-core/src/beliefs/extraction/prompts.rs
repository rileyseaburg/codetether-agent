//! Structured extraction prompts shared by thinker adapters.

/// Build the system and user prompts for a thought.
///
/// # Arguments
/// * `thought_text` - Thought whose factual claims should be extracted.
///
/// # Returns
/// System prompt followed by user prompt.
///
/// # Examples
/// ```
/// use codetether_cognition_core::beliefs::extraction::extraction_prompts;
/// let (_, user) = extraction_prompts("service is healthy");
/// assert!(user.contains("service is healthy"));
/// ```
pub fn extraction_prompts(thought_text: &str) -> (String, String) {
    let system_prompt = "You are a structured belief extractor. \
Given a thought, extract concrete factual claims as structured JSON. \
Return ONLY valid JSON, no markdown fences. \
If no concrete claims exist, return {\"claims\":[]}."
        .to_string();

    let user_prompt = format!(
        "Extract beliefs from this thought:\n\n{thought}\n\n\
Return JSON only: {{ \"claims\": [{{ \"claim\": \"...\", \"belief_key\": \"lowercase-normalized-key\", \
\"confidence\": 0.0-1.0, \"evidence_refs\": [], \"contest_target\": null, \
\"uncertainties\": [] }}] }}",
        thought = thought_text
    );

    (system_prompt, user_prompt)
}
