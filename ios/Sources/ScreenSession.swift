import Foundation

/// Owner-only session receipt; the pairing code is never placed in a URL.
struct ScreenSession: Codable, Equatable {
    let id: UUID
    let code: String
    let pair_expires_at: String
    let expires_at: String
    let interval_seconds: Int

    var pairingExpiry: Date? { ScreenDate.parse(pair_expires_at) }
    var expiry: Date? { ScreenDate.parse(expires_at) }
    var valid: Bool {
        code.count == 12 && code.allSatisfy { "0123456789ABCDEF".contains($0) }
            && pairingExpiry != nil && expiry != nil && (15...300).contains(interval_seconds)
    }
}

/// ISO timestamps may include fractional seconds depending on the relay.
enum ScreenDate {
    static func parse(_ value: String) -> Date? {
        let formatter = ISO8601DateFormatter()
        formatter.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
        if let date = formatter.date(from: value) { return date }
        formatter.formatOptions = [.withInternetDateTime]
        return formatter.date(from: value)
    }
}