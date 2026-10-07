import Foundation

/// Assign streamed images to the current reply, preserving ownership on revisions.
extension ChatModel {
    func appendReplyImages(_ paths: [String]) {
        var claimed = Set(messages.flatMap(\.imagePaths))
        let fresh = paths.filter { claimed.insert($0).inserted }
        guard !fresh.isEmpty else { return }
        if let id = currentReplyID, let index = messages.firstIndex(where: { $0.id == id }) {
            messages[index].imagePaths.append(contentsOf: fresh)
        } else {
            let reply = ChatMessage(role: "assistant", content: "", imagePaths: fresh)
            currentReplyID = reply.id
            messages.append(reply)
        }
    }

    /// Recover tool-result-only images from this turn, not the entire snapshot.
    func reconcileReplyImages(_ snapshot: AgentSession, prompt: String) {
        guard let source = snapshot.messages,
              let start = source.lastIndex(where: { message in
                  message.role == "user" && message.content.compactMap {
                      $0.type == "text" ? $0.text : nil
                  }.joined(separator: "\n") == prompt
              }), let reply = ConversationProjection.messages(Array(source[start...])).last,
              reply.role == "assistant" else { return }
        appendReplyImages(reply.imagePaths)
    }
}