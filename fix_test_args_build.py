with open('src/session/mod.rs', 'r') as f:
    text = f.read()

text = text.replace('''let should_retry = should_force_build_tool_first_retry(
            "build",
            0,
            &tools,
            &session_messages,
            root,
            "If you want, I can patch this next.",
            false,
        );''', '''let should_retry = should_force_build_tool_first_retry(
            "build",
            0,
            &tools,
            &session_messages,
            root,
            "If you want, I can patch this next.",
            false,
            2,
        );''')

text = text.replace('''let should_retry = should_force_build_tool_first_retry(
            "plan",
            0,
            &tools,
            &session_messages,
            root,
            "If you want, I can patch this next.",
            false,
        );''', '''let should_retry = should_force_build_tool_first_retry(
            "plan",
            0,
            &tools,
            &session_messages,
            root,
            "If you want, I can patch this next.",
            false,
            2,
        );''')

with open('src/session/mod.rs', 'w') as f:
    f.write(text)
