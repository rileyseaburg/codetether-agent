import Foundation

/// The relay's bounded event surface; unknown event types fail closed.
struct ScreenEvent: Decodable {
    enum Kind: String, Decodable { case snapshot, capture, delta, done, error, stopped }
    let type: Kind
    let seq: Int
    let text: String?
    let status: String?
    let captured_at: String?
}

/// Current analysis only, never a growing history of screenshots or answers.
struct ScreenResponse {
    private(set) var sequence = -1
    private(set) var text = ""
    private(set) var status = "Waiting for Windows pairing"
    private(set) var capturedAt: String?

    mutating func apply(_ event: ScreenEvent) {
        guard event.type == .snapshot || event.seq > sequence else { return }
        sequence = event.seq
        switch event.type {
        case .snapshot, .capture:
            text = event.text ?? ""
            capturedAt = event.captured_at
            status = event.status ?? (event.type == .capture ? "Analyzing screen" : "Waiting")
        case .delta:
            text += event.text ?? ""
            status = event.status ?? "Receiving analysis"
        case .done: text = event.text ?? text; status = event.status ?? "Analysis ready"
        case .error: text = event.text ?? "Analysis unavailable"; status = "Analysis error"
        case .stopped: status = event.status ?? "Stopped"
        }
        if let captured = event.captured_at { capturedAt = captured }
    }
}