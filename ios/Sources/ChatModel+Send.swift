import Foundation

extension ChatModel {
    func send() {
        let text = draft.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !busy, !text.isEmpty || !attachments.isEmpty else { return }
        let message = ChatMessage(role: "user", content: text)
        messages.append(message)
        draft = ""
        busy = true
        error = nil
        replyForSpeech = nil
        activeAssistantItem = nil; completedItems = []; currentReplyID = nil
        requestTask = Task {
            defer { busy = false; requestTask = nil; activeAssistantItem = nil }
            do {
                let reply = try await runAgentTurn(text.isEmpty ? "Describe the attached image." : text)
                try Task.checkCancellation()
                let text = TranscriptPresentation.displayText(reply.text)
                upsertReply(text)
                replyForSpeech = text
                attachments = []
                ChatReceipt.write(model: "codetether-agent/session", messageCount: messages.count)
            } catch {
                if completedItems.isEmpty {
                    messages.removeAll { $0.id == message.id }
                    if draft.isEmpty { draft = text }
                }
                if !Task.isCancelled {
                    self.error = (error as? ClientError)?.localizedDescription ?? "Could not get a reply. Your message is ready to retry."
                }
            }
        }
    }
}
