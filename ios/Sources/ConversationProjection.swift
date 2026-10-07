import Foundation

enum ConversationProjection {
    /// Server history is retained; the UI shows one assistant answer per user turn.
    static func messages(_ source: [AgentSession.Message]) -> [ChatMessage] {
        var result: [ChatMessage] = []
        var seenImages = Set<String>()
        for message in source {
            guard ["user", "assistant", "tool"].contains(message.role) else { continue }
            let raw = message.content.compactMap { $0.type == "text" ? $0.text : nil }.joined(separator: "\n")
            guard !TranscriptPresentation.isRuntimeContinuation(raw) else { continue }
            // First appearance owns an image. Later path mentions must not repeat it.
            let images = message.imagePaths.filter { seenImages.insert($0).inserted }
            let text = message.role == "tool" || raw.isEmpty ? "" : TranscriptPresentation.displayText(raw)
            guard !text.isEmpty || !images.isEmpty else { continue }
            let role = message.role == "user" && !raw.isEmpty ? "user" : "assistant"
            if role == "assistant", let last = result.last, last.role == "assistant" {
                result[result.count - 1] = ChatMessage(id: last.id, role: role,
                    content: text.isEmpty ? last.content : text, imagePaths: last.imagePaths + images)
            } else {
                result.append(ChatMessage(role: role, content: text, imagePaths: images))
            }
        }
        return result
    }
}