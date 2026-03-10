import os

with open('src/session/mod.rs', 'r') as f:
    text = f.read()

text = text.replace('is_prompt_too_long_error(&e, &self.id)', 'is_prompt_too_long_error(&e)')

text = text.replace('''if should_force_build_tool_first_retry(
                &self.agent,
                build_mode_tool_retry_count,
                &tool_definitions,
                &self.messages,
                &cwd,
                &assistant_text,
                !tool_calls.is_empty(),
            )''', '''if should_force_build_tool_first_retry(
                &self.agent,
                build_mode_tool_retry_count,
                &tool_definitions,
                &self.messages,
                &cwd,
                &assistant_text,
                !tool_calls.is_empty(),
                BUILD_MODE_TOOL_FIRST_MAX_RETRIES,
            )''')

text = text.replace('''if should_force_build_tool_first_retry(
                &self.agent,
                build_mode_tool_retry_count,
                &tool_definitions,
                &self.messages,
                &cwd,
                &assistant_text,
                has_tools,
            )''', '''if should_force_build_tool_first_retry(
                &self.agent,
                build_mode_tool_retry_count,
                &tool_definitions,
                &self.messages,
                &cwd,
                &assistant_text,
                has_tools,
                BUILD_MODE_TOOL_FIRST_MAX_RETRIES,
            )''')

text = text.replace('''if should_retry_missing_native_tool_call(
                selected_provider,
                &model,
                native_tool_promise_retry_count,
                &tool_definitions,
                &assistant_text,
                !tool_calls.is_empty(),
            )''', '''if should_retry_missing_native_tool_call(
                selected_provider,
                &model,
                native_tool_promise_retry_count,
                &tool_definitions,
                &assistant_text,
                !tool_calls.is_empty(),
                NATIVE_TOOL_PROMISE_RETRY_MAX_RETRIES,
            )''')

text = text.replace('''if should_retry_missing_native_tool_call(
                selected_provider,
                &model,
                native_tool_promise_retry_count,
                &tool_definitions,
                &assistant_text,
                has_tools,
            )''', '''if should_retry_missing_native_tool_call(
                selected_provider,
                &model,
                native_tool_promise_retry_count,
                &tool_definitions,
                &assistant_text,
                has_tools,
                NATIVE_TOOL_PROMISE_RETRY_MAX_RETRIES,
            )''')

with open('src/session/mod.rs', 'w') as f:
    f.write(text)

