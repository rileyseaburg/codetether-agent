import Foundation

/// Create one server session per conversation; stale responses cannot replace its ID.
extension ChatModel {
    private struct CreateSession: Encodable { let title: String; let agent = "build" }

    func ensureSession(token: String, title: String) async throws -> String {
        if let sessionID { return sessionID }
        let generation = conversationID
        if sessionCreationID != generation {
            sessionCreationID = generation
            sessionCreation = Task {
                let session: AgentSession = try await client.post("api/session", token: token,
                    body: CreateSession(title: title))
                return session.id
            }
        }
        guard let creation = sessionCreation else { throw CancellationError() }
        defer {
            if sessionCreationID == generation { sessionCreation = nil; sessionCreationID = nil }
        }
        let id = try await creation.value
        try Task.checkCancellation()
        guard conversationID == generation else { throw CancellationError() }
        guard UUID(uuidString: id) != nil else { throw ClientError.invalidResponse }
        sessionID = id
        UserDefaults.standard.set(id, forKey: "agent.session")
        return id
    }

    @discardableResult
    func newConversation() async -> Bool {
        clear()
        let generation = conversationID
        loading = true
        defer { if generation == conversationID { loading = false } }
        do {
            guard let token = try TokenStore.read() else { throw ClientError.missingToken }
            _ = try await ensureSession(token: token, title: "iPhone: New conversation")
            return generation == conversationID
        } catch {
            if generation == conversationID { self.error = "Could not create a new session. Retry New chat." }
            return false
        }
    }
}