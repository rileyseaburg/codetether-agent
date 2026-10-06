import Foundation

enum ConversationProjection {
    /// Server history is retained; the UI shows one assistant answer per user turn.
    static func messages(_ source: [AgentSession.Message]) -> [ChatMessage] {
        var result: [ChatMessage] = []
        for message in source {
            guard ["user", "assistant"].contains(message.role) else { continue }
            let raw = message.content.compactMap { $0.type == "text" ? $0.text : nil }.joined(separator: "\n")
            guard !raw.isEmpty, !TranscriptPresentation.isRuntimeContinuation(raw) else { continue }
            let text = TranscriptPresentation.displayText(raw)
            if message.role == "assistant", let last = result.last, last.role == "assistant" {
                result[result.count - 1] = ChatMessage(id: last.id, role: "assistant", content: text)
            } else {
                result.append(ChatMessage(role: message.role, content: text))
            }
        }
        return result
    }
}
