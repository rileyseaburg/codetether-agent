use std::collections::HashSet;
use std::sync::Arc;

use anyhow;
use codetether_agent::mcp::{CallToolResult, McpServer, ToolContent};
use serde_json::json;

#[tokio::test]
async fn mcp_local_server_lists_tools_without_stdio_transport() {
    // This should not spawn stdio reader/writer threads (which lock stdout) and should be
    // safe to use from CLI commands that need tool metadata.
    let server = McpServer::new_local();
    server.setup_tools_public().await;

    let tools = server.get_all_tool_metadata().await;
    let names: HashSet<&str> = tools.iter().map(|t| t.name.as_str()).collect();

    // Minimum expected tool surface.
    for expected in [
        "run_command",
        "read_file",
        "write_file",
        "list_directory",
        "search_files",
        "grep_search",
    ] {
        assert!(names.contains(expected), "missing tool: {expected}");
    }
}

/// Test US-002: Verify dynamic tool registration works
/// - Call register_tool with new tool metadata
/// - Call tools/list and verify new tool appears
/// - Tool metadata matches registered data
#[tokio::test]
async fn mcp_dynamic_tool_registration() {
    // Create a local MCP server
    let server = McpServer::new_local();

    // Register a new tool dynamically at runtime
    let tool_name = "dynamic_echo";
    let tool_description = "Echoes the input message with a prefix";
    let input_schema = json!({
        "type": "object",
        "properties": {
            "message": {
                "type": "string",
                "description": "Message to echo"
            },
            "repeat": {
                "type": "integer",
                "description": "Number of times to repeat",
                "default": 1
            }
        },
        "required": ["message"]
    });

    server
        .register_tool(
            tool_name,
            tool_description,
            input_schema.clone(),
            Arc::new(|args| {
                let message = args
                    .get("message")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing message"))?;

                let repeat = args.get("repeat").and_then(|v| v.as_i64()).unwrap_or(1) as usize;

                let result = message.repeat(repeat);
                Ok(CallToolResult {
                    content: vec![ToolContent::Text { text: result }],
                    is_error: false,
                })
            }),
        )
        .await;

    // Get all tool metadata (simulating tools/list API)
    let tools = server.get_all_tool_metadata().await;
    let names: HashSet<&str> = tools.iter().map(|t| t.name.as_str()).collect();

    // Verify the new tool appears in the list
    assert!(
        names.contains(tool_name),
        "dynamic tool should appear in tools/list"
    );

    // Verify the tool metadata matches registered data
    let tool_meta = tools
        .iter()
        .find(|t| t.name == tool_name)
        .expect("tool should exist");
    assert_eq!(tool_meta.name, tool_name, "tool name should match");
    assert_eq!(
        tool_meta.description.as_deref(),
        Some(tool_description),
        "tool description should match"
    );
    assert_eq!(
        tool_meta.input_schema, input_schema,
        "tool input_schema should match"
    );

    // Also verify the tool can be executed
    let result = server
        .call_tool_direct(tool_name, json!({"message": "Hello, World!", "repeat": 2}))
        .await
        .expect("tool should execute");

    let output = match &result.content[0] {
        ToolContent::Text { text } => text.clone(),
        _ => panic!("expected text content"),
    };
    assert_eq!(
        output, "Hello, World!Hello, World!",
        "tool should echo the message"
    );
}
