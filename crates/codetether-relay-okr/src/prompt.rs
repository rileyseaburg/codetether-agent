//! Task and quantitative constraints for model-authored relay OKRs.
pub(crate) fn user(task: &str, agent_count: usize) -> String {
    format!(
        "Task:\n{task}\n\nTeam size: {agent_count}\n\n\
                         Propose ONE objective and 3-7 measurable key results for executing this task via an AI relay.\n\
                         Key results must be quantitative (numeric target_value + unit).\n\n\
                         Return JSON ONLY (no markdown):\n\
                         {{\n  \"title\": \"...\",\n  \"description\": \"...\",\n  \"key_results\": [\n    {{\"title\":\"...\",\"target_value\":123,\"unit\":\"%|count|tests|files|items\"}}\n  ]\n}}\n\n\
                         Rules:\n\
                         - Avoid vague KRs like 'do better'\n\
                         - Prefer engineering outcomes (tests passing, endpoints implemented, docs updated, errors=0)\n\
                         - If unsure about a unit, use 'count'"
    )
}
