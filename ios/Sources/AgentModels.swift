import Foundation

struct AgentSession: Decodable {
    let id: String
    let messages: [Message]?
    struct Message: Decodable { let role: String; let content: [Part] }
    struct Part: Decodable {
        let type: String
        let text: String?
        let name: String?
        let content: String?
    }
    var transcript: [ChatMessage] {
        ConversationProjection.messages(messages ?? [])
    }
    var toolNames: [String] {
        Array(Set((messages ?? []).flatMap(\.content).compactMap { $0.type == "tool_call" ? $0.name : nil })).sorted()
    }
}

struct AgentReply: Decodable, Sendable { let text: String; let session_id: String }
struct AgentFrame: Decodable, Sendable {
    let type: String
    let result: AgentReply?
    let message: String?
    let event: Event?
    struct Event: Decodable, Sendable { let kind: String; let payload: Payload? }
    struct Payload: Decodable, Sendable {
        let text: String?; let name: String?; let success: Bool?; let metadata: Metadata?
        let item_id: String?; let item_type: String?
    }
    struct Metadata: Decodable, Sendable { let saved_path: String? }
}

@MainActor protocol AgentTransport {
    func prompt(sessionID: String, message: String, status: @escaping (AgentFrame.Event) -> Void) async throws -> AgentReply
}
