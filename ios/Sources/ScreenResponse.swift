import Foundation

/// Current analysis only, never a growing history of screenshots or answers.
struct ScreenResponse {
    private(set) var sequence = -1
    private(set) var text = ""
    private(set) var status = "waiting"
    private(set) var capturedAt: String?

    var canRequestFrame: Bool { ["paired", "paused", "ready", "error"].contains(status) }
    var isPaired: Bool { canRequestFrame || ["requested", "analyzing"].contains(status) }
    var statusTitle: String {
        switch status {
        case "waiting": return "Waiting for Windows pairing"
        case "paired": return "Windows paired — select a monitor locally"
        case "requested": return "Waiting for a fresh screenshot"
        case "analyzing": return "Analyzing screen"
        case "ready": return "Analysis ready"
        case "error": return "Analysis error"
        case "paused": return "Capture paused on Windows"
        case "stopped": return "Session stopped"
        default: return "Waiting for screen status"
        }
    }
    mutating func apply(_ event: ScreenEvent) {
        guard event.type == .snapshot || event.seq > sequence else { return }
        sequence = event.seq
        switch event.type {
        case .snapshot, .capture:
            text = event.text ?? ""
            capturedAt = event.captured_at
            status = event.status ?? (event.type == .capture ? "analyzing" : "waiting")
        case .delta:
            text += event.text ?? ""
            status = "analyzing"
        case .done: text = event.text ?? text; status = "ready"
        case .error: text = event.text ?? "Analysis unavailable"; status = "error"
        case .stopped: status = "stopped"
        }
        if let captured = event.captured_at { capturedAt = captured }
    }
}
