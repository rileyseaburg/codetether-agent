use std::collections::HashSet;

use codetether_agent::mcp::McpServer;

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
