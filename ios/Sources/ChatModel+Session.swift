import Foundation

extension ChatModel {
    func runAgentTurn(_ text: String, turn: UUID, model: String) async throws -> AgentReply {
        try requireTurn(turn)
        guard let token = try TokenStore.read() else { throw ClientError.missingToken }
        let id = try await ensureSession(token: token, title: "iPhone: " + text.prefix(60))
        try requireTurn(turn)
        let prompt = try await attachmentPrompt(text, token: token, turn: turn)
        try requireTurn(turn)
        agentStatus = "Agent working…"
        let reply = try await agent.prompt(sessionID: id, message: prompt, model: model) { event in
            guard self.sessionID == id, self.activeTurnID == turn else { return }
            self.receive(event)
        }
        try requireTurn(turn)
        guard reply.session_id == id else { throw ClientError.invalidResponse }
        if let snapshot: AgentSession = try? await client.get("api/session/\(reply.session_id)", token: token) {
            try requireTurn(turn)
            tools = snapshot.toolNames
            reconcileReplyImages(snapshot, prompt: prompt)
        }
        try requireTurn(turn)
        agentStatus = "Agent finished"
        AgentReceipt.write(session: reply.session_id, tools: tools)
        return reply
    }
    func restoreSession() async {
        guard let id = sessionID, !busy else { return }
        let generation = conversationID
        let lastMessage = messages.last?.id
        do {
            guard let token = try TokenStore.read() else { throw ClientError.missingToken }
            let snapshot: AgentSession = try await client.get("api/session/\(id)", token: token)
            guard conversationID == generation, sessionID == id, !busy,
                  messages.last?.id == lastMessage else { return }
            messages = snapshot.transcript
            tools = snapshot.toolNames
            currentReplyID = nil
        } catch {
            if conversationID == generation { self.error = "Could not restore the conversation. Use New chat to start again." }
        }
    }
}