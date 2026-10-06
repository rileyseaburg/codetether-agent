import Foundation

/// Records successful chat delivery without saving prompts or replies.
enum ChatReceipt {
    private struct Receipt: Encodable {
        let endpoint: String
        let model: String
        let messageCount: Int
        let checkedAt: Date
    }
    static func write(model: String, messageCount: Int) {
        let receipt = Receipt(endpoint: "https://server.codetether.run/v1/chat/completions",
                              model: model, messageCount: messageCount, checkedAt: Date())
        let encoder = JSONEncoder()
        encoder.dateEncodingStrategy = .iso8601
        if let data = try? encoder.encode(receipt) {
            try? data.write(to: URL.documentsDirectory.appendingPathComponent("chat-receipt.json"),
                            options: [.atomic, .completeFileProtection])
        }
    }
}
