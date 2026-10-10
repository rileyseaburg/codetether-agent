import Foundation
import Combine

/// Memory-only question draft and delivery state, never an agent conversation.
@MainActor
final class ScreenQuestionState: ObservableObject {
    @Published var draft = ""
    @Published var submitting = false
    @Published var waitingForFrame = false
    @Published var error: String?
    @Published var notice: String?
    var generation = UUID()
    var task: Task<Void, Never>?

    func cancel(clearDraft: Bool = false) {
        generation = UUID()
        task?.cancel()
        task = nil
        notice = submitting ? "Delivery was interrupted. Reconnect before retrying; the question may already be queued." : nil
        submitting = false
        waitingForFrame = false
        error = nil
        if clearDraft { draft = ""; notice = nil }
    }

    func receive(_ event: ScreenEvent) {
        if event.type == .snapshot && event.status == "requested" {
            waitingForFrame = true
            notice = "Waiting for a fresh screenshot from Windows…"
        } else if event.type != .delta {
            waitingForFrame = false
            notice = nil
        }
    }

    deinit { task?.cancel() }
}
