import sys

def fix_file(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # Define the imports to prepend
    header = """pub mod helper;
use self::helper::text::{extract_text_content, latest_user_text, extract_candidate_file_paths, truncate_with_ellipsis, role_label};
use self::helper::stream::collect_stream_completion_with_events;
use self::helper::markup::{extract_markup_tool_calls, normalize_textual_tool_calls, tool_call_markup_re};
use self::helper::build::{is_build_agent, should_force_build_tool_first_retry, build_request_requires_tool, assistant_offered_next_step, is_affirmative_build_followup, looks_like_build_execution_request, looks_like_proposed_change};
use self::helper::token::{context_window_for_model, session_completion_max_tokens, estimate_tokens_for_part, estimate_tokens_for_messages, estimate_tokens_for_tools, estimate_request_tokens};
use self::helper::error::{messages_to_rlm_context, is_prompt_too_long_error, is_retryable_upstream_error};
use self::helper::provider::{provider_has_flaky_native_tool_calling, assistant_claims_imminent_tool_use, should_retry_missing_native_tool_call, choose_default_provider, resolve_provider_for_session_request, prefers_temperature_one, known_good_router_candidates, choose_router_target, enrich_tool_input_with_runtime_context, is_interactive_tool, is_local_cuda_provider, local_cuda_light_system_prompt};
"""

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

    lines = content.splitlines(keepends=True)
    new_lines = []
    skip_until_brace_match = False
    brace_count = 0
    in_impl = False

    for line in lines:
        trimmed = line.strip()
        
        # Track impl blocks
        if trimmed.startswith("impl ") and "{" in line:
             in_impl = True
             # If it's a multiline block start, brace count tracking begins
             brace_count += line.count('{') - line.count('}')
        elif in_impl:
             brace_count += line.count('{') - line.count('}')
             if brace_count <= 0:
                 in_impl = False
        
        # Track skips for standalone functions
        if skip_until_brace_match:
            brace_count_skip += line.count('{') - line.count('}')
            if brace_count_skip <= 0:
                skip_until_brace_match = False
            continue

        if not in_impl and trimmed.startswith("fn "):
            # Extract function name correctly even with generics
            fn_name = trimmed.replace("fn ", "").split("(")[0].split("<")[0].strip()
            if fn_name in moved_fns:
                skip_until_brace_match = True
                brace_count_skip = line.count('{') - line.count('}')
                if brace_count_skip <= 0:
                    skip_until_brace_match = False
                continue

        new_lines.append(line)

    # Re-insert the header after mod comments if any
    output = []
    i = 0
    while i < len(new_lines) and (new_lines[i].startswith("//") or new_lines[i].strip() == ""):
        output.append(new_lines[i])
        i += 1
    
    output.append(header)
    output.extend(new_lines[i:])

    with open(file_path, 'w', encoding='utf-8') as f:
        f.writelines(output)

if __name__ == "__main__":
    fix_file('src/session/mod.rs')
