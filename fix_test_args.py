with open('src/session/mod.rs', 'r') as f:
    text = f.read()

text = text.replace('''assert!(should_retry_missing_native_tool_call(
            "minimax",
            "MiniMax-M2.5",
            0,
            &tools,
            "I'll use bash to inspect the repository first.",
            false,
        ));''', '''assert!(should_retry_missing_native_tool_call(
            "minimax",
            "MiniMax-M2.5",
            0,
            &tools,
            "I'll use bash to inspect the repository first.",
            false,
            1,
        ));''')

text = text.replace('''assert!(!should_retry_missing_native_tool_call(
            "anthropic",
            "claude-sonnet-4",
            0,
            &tools,
            "I'll use bash to inspect the repository first.",
            false,
        ));''', '''assert!(!should_retry_missing_native_tool_call(
            "anthropic",
            "claude-sonnet-4",
            0,
            &tools,
            "I'll use bash to inspect the repository first.",
            false,
            1,
        ));''')

text = text.replace('''assert!(!should_retry_missing_native_tool_call(
            "minimax",
            "MiniMax-M2.5",
            1,
            &tools,
            "I'll use bash to inspect the repository first.",
            false,
        ));''', '''assert!(!should_retry_missing_native_tool_call(
            "minimax",
            "MiniMax-M2.5",
            1,
            &tools,
            "I'll use bash to inspect the repository first.",
            false,
            1,
        ));''')

with open('src/session/mod.rs', 'w') as f:
    f.write(text)
