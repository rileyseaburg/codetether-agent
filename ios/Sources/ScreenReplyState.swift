import Foundation
import Combine

/// Memory-only typed-reply draft and delivery state; one queued reply at a time.
@MainActor
final class ScreenReplyState: ObservableObject {
    @Published var draft = ""
    @Published var submitting = false
    @Published var error: String?
    @Published var notice: String?
    var generation = UUID()
    var task: Task<Void, Never>?

    func cancel(clearDraft: Bool = false) {
        generation = UUID()
        task?.cancel()
        task = nil
        notice = submitting ? "Delivery was interrupted. The reply may already be queued." : nil
        submitting = false
        error = nil
        if clearDraft { draft = ""; notice = nil }
    }

    deinit { task?.cancel() }
}
