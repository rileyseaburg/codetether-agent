import sys

def fix_file(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    moved_fns = [
        "is_interactive_tool", "is_local_cuda_provider", "local_cuda_light_system_prompt",
        "enrich_tool_input_with_runtime_context", "is_build_agent", "extract_text_content",
        "collect_stream_completion_with_events", "tool_call_markup_re", "extract_markup_tool_calls",
        "normalize_textual_tool_calls", "is_codesearch_no_match_output", 
        "looks_like_build_execution_request", "is_affirmative_build_followup", 
        "looks_like_proposed_change", "assistant_offered_next_step", "build_request_requires_tool",
        "should_force_build_tool_first_retry", "provider_has_flaky_native_tool_calling",
        "assistant_claims_imminent_tool_use", "should_retry_missing_native_tool_call",
        "choose_default_provider", "resolve_provider_for_session_request", "prefers_temperature_one",
        "context_window_for_model", "session_completion_max_tokens", "estimate_tokens_for_part",
        "estimate_tokens_for_messages", "estimate_tokens_for_tools", "estimate_request_tokens",
        "role_label", "inject_tool_prompt", "list_tools_bootstrap_definition",
        "list_tools_bootstrap_output", "stub_marker_in_text", "detect_stub_in_tool_input",
        "normalize_tool_call_for_execution", "latest_user_text", "extract_candidate_file_paths",
        "build_proactive_lsp_context_message", "messages_to_rlm_context", "is_prompt_too_long_error",
        "is_retryable_upstream_error", "known_good_router_candidates", "choose_router_target",
        "list_sessions", "list_sessions_for_directory", "list_sessions_paged", "truncate_with_ellipsis"
    ]

    new_lines = []
    skip_fn = False
    seen_brace = False
    brace_count = 0
    in_impl = 0

    for line in lines:
        trimmed = line.strip()
        
        # Track impl to avoid false pos
        if (line.startswith("impl ") or (line.startswith("pub ") and "impl " in line)) and "{" in line:
            in_impl += 1

        if skip_fn:
            if "{" in line:
                seen_brace = True
                brace_count += line.count('{')
            if "}" in line:
                brace_count -= line.count('}')
                
            if seen_brace and brace_count <= 0:
                skip_fn = False
            continue

        is_root_fn = (line.startswith("fn ") or line.startswith("async fn ") or line.startswith("pub fn "))
        if in_impl == 0 and is_root_fn:
            prefix = ""
            if line.startswith("fn "): prefix = "fn "
            elif line.startswith("async fn "): prefix = "async fn "
            elif line.startswith("pub fn "): prefix = "pub fn "
            
            fn_name = line.replace(prefix, "").split("(")[0].split("<")[0].strip()
            if fn_name in moved_fns:
                skip_fn = True
                seen_brace = "{" in line
                if seen_brace:
                    brace_count = line.count('{') - line.count('}')
                    if brace_count <= 0:
                        skip_fn = False
                else:
                    brace_count = 0
                continue

        new_lines.append(line)

        if in_impl > 0 and (trimmed == "}" or trimmed.endswith("}")):
            if line.count('}') > line.count('{'):
                in_impl -= 1

    # Insert helper and imports at top
    final_output = ["pub mod helper;\n"]
    
    helper_imports = """
use self::helper::text::{extract_text_content, latest_user_text, extract_candidate_file_paths, truncate_with_ellipsis, role_label};
use self::helper::stream::collect_stream_completion_with_events;
use self::helper::markup::{extract_markup_tool_calls, normalize_textual_tool_calls, tool_call_markup_re};
use self::helper::build::{is_build_agent, should_force_build_tool_first_retry, build_request_requires_tool, assistant_offered_next_step, is_affirmative_build_followup, looks_like_build_execution_request, looks_like_proposed_change};
use self::helper::token::{context_window_for_model, session_completion_max_tokens, estimate_tokens_for_part, estimate_tokens_for_messages, estimate_tokens_for_tools, estimate_request_tokens};
use self::helper::error::{messages_to_rlm_context, is_prompt_too_long_error, is_retryable_upstream_error};
use self::helper::provider::{provider_has_flaky_native_tool_calling, assistant_claims_imminent_tool_use, should_retry_missing_native_tool_call, choose_default_provider, resolve_provider_for_session_request, prefers_temperature_one, known_good_router_candidates, choose_router_target, enrich_tool_input_with_runtime_context, is_interactive_tool, is_local_cuda_provider, local_cuda_light_system_prompt};
"""
    
    inserted_imports = False
    for line in new_lines:
        if line.startswith("pub mod helper;"):
            continue # Skip if already there from previous runs
        if not inserted_imports and "use crate::cognition::tool_router" in line:
            final_output.append(line)
            final_output.append(helper_imports)
            inserted_imports = True
        else:
            final_output.append(line)

    with open(file_path, 'w', encoding='utf-8') as f:
        f.writelines(final_output)

if __name__ == "__main__":
    fix_file('src/session/mod.rs')
