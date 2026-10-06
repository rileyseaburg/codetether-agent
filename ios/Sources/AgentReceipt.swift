import Foundation

enum AgentReceipt {
    private struct Receipt: Encodable {
        let sessionID: String
        let tools: [String]
        let checkedAt = Date()
    }
    static func write(session: String, tools: [String]) {
        let encoder = JSONEncoder()
        encoder.dateEncodingStrategy = .iso8601
        if let data = try? encoder.encode(Receipt(sessionID: session, tools: tools)) {
            try? data.write(to: URL.documentsDirectory.appendingPathComponent("agent-receipt.json"),
                            options: [.atomic, .completeFileProtection])
        }
    }
}
