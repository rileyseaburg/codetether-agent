import Foundation

struct ChatMessage: Identifiable, Codable {
    var id = UUID()
    let role: String
    let content: String
    /// Presentation-only paths owned by this message; excluded from chat API payloads.
    var imagePaths: [String] = []
    enum CodingKeys: String, CodingKey { case role, content }
}

struct ChatRequest: Encodable {
    let model: String
    let messages: [ChatMessage]
    let stream = false
    let max_tokens = 4096
}

struct ChatResponse: Decodable {
    struct Choice: Decodable { let message: Message }
    struct Message: Decodable { let content: String? }
    let choices: [Choice]
    let model: String
}

struct ModelCatalog: Decodable {
    struct Entry: Decodable, Identifiable {
        let id: String
        let owned_by: String?
    }
    let data: [Entry]
}

struct ChatConfiguration: Decodable { let default_model: String? }