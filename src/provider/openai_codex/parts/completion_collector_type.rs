#[derive(Default)]
struct ToolAccumulator {
    id: String,
    name: String,
    arguments: String,
}

#[derive(Default)]
struct CompletionCollector {
    text: String,
    reasoning_signature: Option<String>,
    tools: Vec<ToolAccumulator>,
    tool_indexes: HashMap<String, usize>,
    usage: Usage,
    saw_done: bool,
}
