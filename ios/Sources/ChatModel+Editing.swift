import Foundation

/// Save edits as a new branch; neither the original session nor its later replies is overwritten.
extension ChatModel {
    private struct ForkRequest: Encodable { let before_message: Int; let expected_text: String }

    func editAndResend(_ message: ChatMessage, text: String) async throws {
        let text = text.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !busy, !loading, let sessionID, !text.isEmpty else {
            throw ClientError.agent("Wait for the current response and enter a message before saving.")
        }
        let generation = conversationID
        loading = true
        defer { if conversationID == generation { loading = false } }
        guard let token = try TokenStore.read() else { throw ClientError.missingToken }
        let snapshot: AgentSession = try await client.get("api/session/\(sessionID)", token: token)
        guard conversationID == generation else { throw CancellationError() }
        let target = try MessageEditContext.find(message, in: messages, snapshot: snapshot)
        let branch: AgentSession = try await client.post("api/session/\(sessionID)/fork", token: token,
            body: ForkRequest(before_message: target.index, expected_text: target.original))
        try Task.checkCancellation()
        guard conversationID == generation, UUID(uuidString: branch.id) != nil,
              branch.id != sessionID else { throw ClientError.invalidResponse }
        clear()
        self.sessionID = branch.id
        UserDefaults.standard.set(branch.id, forKey: "agent.session")
        messages = branch.transcript
        guard let turn = beginTurn(text, owner: .chat) else { throw ClientError.invalidResponse }
        if let index = messages.firstIndex(where: { $0.id == turn.message.id }) {
            messages[index].imagePaths = ImageReferencePaths.extract(target.attachmentSuffix)
        }
        requestTask = Task { await executeTurn(turn, prompt: text + target.attachmentSuffix) }
    }
}