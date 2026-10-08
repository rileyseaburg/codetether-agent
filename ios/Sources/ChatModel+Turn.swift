import Foundation

/// Shared Chat/Voice turn lifecycle; stale completions cannot mutate a new conversation.
extension ChatModel {
    func beginTurn(_ text: String, owner: ReplySpeechOwner) -> ChatTurn? {
        let text = text.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !busy, !loading, !text.isEmpty || !attachments.isEmpty else { return nil }
        let turn = ChatTurn(message: ChatMessage(role: "user", content: text), model: selectedModel)
        activeTurnID = turn.id
        messages.append(turn.message)
        draft = ""; busy = true; error = nil
        prepareReplySpeech(for: owner)
        activeAssistantItem = nil; completedItems = []; currentReplyID = nil
        return turn
    }

    func executeTurn(_ turn: ChatTurn, prompt override: String? = nil) async -> String? {
        defer { finishTurn(turn.id) }
        do {
            let prompt = override ?? (turn.message.content.isEmpty ? "Describe the attached image." : turn.message.content)
            let reply = try await runAgentTurn(prompt, turn: turn.id, model: turn.model)
            try requireTurn(turn.id)
            let text = TranscriptPresentation.displayText(reply.text)
            upsertReply(text); publishReplySpeech(text); attachments = []
            ChatReceipt.write(model: "codetether-agent/session", messageCount: messages.count)
            return text
        } catch {
            guard activeTurnID == turn.id else { return nil }
            if completedItems.isEmpty {
                messages.removeAll { $0.id == turn.message.id }
                if draft.isEmpty { draft = turn.message.content }
            }
            if !Task.isCancelled {
                self.error = (error as? ClientError)?.localizedDescription ?? "Could not get a reply. Your message is ready to retry."
            }
            return nil
        }
    }
}