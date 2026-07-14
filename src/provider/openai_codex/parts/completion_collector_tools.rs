impl CompletionCollector {
    fn start_tool(&mut self, id: String, name: String) {
        let next = self.tools.len();
        let index = *self.tool_indexes.entry(id.clone()).or_insert(next);
        if index == next {
            self.tools.push(ToolAccumulator {
                id,
                name,
                arguments: String::new(),
            });
        } else if self.tools[index].name == "tool" {
            self.tools[index].name = name;
        }
    }

    fn append_tool_arguments(&mut self, id: String, arguments: String) {
        if let Some(index) = self.tool_indexes.get(&id).copied() {
            self.tools[index].arguments.push_str(&arguments);
            return;
        }
        let index = self.tools.len();
        self.tool_indexes.insert(id.clone(), index);
        self.tools.push(ToolAccumulator {
            id,
            name: "tool".to_string(),
            arguments,
        });
    }
}
