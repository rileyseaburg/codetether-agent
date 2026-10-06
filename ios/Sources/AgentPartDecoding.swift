import Foundation

extension AgentSession.Part {
    private enum Keys: String, CodingKey { case type, text, name, content }
    /// Retain display text and image references, not reasoning or full tool logs.
    init(from decoder: Decoder) throws {
        let fields = try decoder.container(keyedBy: Keys.self)
        type = try fields.decode(String.self, forKey: .type)
        text = type == "text" ? try fields.decodeIfPresent(String.self, forKey: .text) : nil
        name = type == "tool_call" ? try fields.decodeIfPresent(String.self, forKey: .name) : nil
        if type == "tool_result", let output = try fields.decodeIfPresent(String.self, forKey: .content) {
            content = ImageReferencePaths.extract(output).joined(separator: "\n")
        } else {
            content = nil
        }
    }
}
