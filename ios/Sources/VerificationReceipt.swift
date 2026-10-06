import Foundation

/// Credential-free evidence of a successful refresh, retrievable with devicectl.
enum VerificationReceipt {
    private struct Receipt: Encodable {
        let endpoint: String
        let version: String
        let agentCount: Int
        let checkedAt: Date
    }

    static func write(version: String, agentCount: Int) {
        let receipt = Receipt(endpoint: ServerClient.origin.absoluteString,
                              version: version, agentCount: agentCount, checkedAt: Date())
        let encoder = JSONEncoder()
        encoder.dateEncodingStrategy = .iso8601
        let url = URL.documentsDirectory.appendingPathComponent("connection-receipt.json")
        if let data = try? encoder.encode(receipt) {
            try? data.write(to: url, options: [.atomic, .completeFileProtection])
        }
    }
}
