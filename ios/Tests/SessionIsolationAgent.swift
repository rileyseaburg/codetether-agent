import Foundation
@testable import CodeTether

/// Records routing and can deliberately deliver an old response after cancellation.
@MainActor
final class SessionIsolationAgent: AgentTransport {
    var sessions: [String] = []
    var models: [String] = []
    var onFirstStarted: (() -> Void)?
    var pending: CheckedContinuation<AgentReply, Error>?
    var oldEvents: ((AgentFrame.Event) -> Void)?

    func prompt(sessionID: String, message: String, model: String, status: @escaping (AgentFrame.Event) -> Void) async throws -> AgentReply {
        sessions.append(sessionID)
        models.append(model)
        if sessions.count == 1, let onFirstStarted {
            oldEvents = status
            return try await withCheckedThrowingContinuation {
                pending = $0
                onFirstStarted()
            }
        }
        return AgentReply(text: "Reply: " + message, session_id: sessionID)
    }
}