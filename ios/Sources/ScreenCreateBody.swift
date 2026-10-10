import Foundation

/// Frozen session configuration sent once; later chat model changes do not affect it.
struct ScreenCreateBody: Encodable {
    let model: String
    let prompt: String
    let interval_seconds: Int

    var valid: Bool {
        ScreenInput.model(model) && ScreenInput.text(prompt) && (15...300).contains(interval_seconds)
    }
}

/// DELETE must explicitly acknowledge revocation before the owner handle is dropped.
struct ScreenStopReceipt: Decodable {
    let stopped: Bool
}
