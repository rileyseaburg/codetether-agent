import Foundation

/// Restartable last-word deadline; injectable clock for deterministic multi-turn tests.
@MainActor
final class VoiceSilenceTimer {
    var interval: TimeInterval = 3 { didSet { schedule() } }
    var now: () -> TimeInterval = { ProcessInfo.processInfo.systemUptime }
    var onExpired: (() -> Void)?
    private var lastWord: TimeInterval?
    private var task: Task<Void, Never>?

    func wordHeard() {
        lastWord = now()
        schedule()
    }

    func cancel() {
        lastWord = nil
        task?.cancel(); task = nil
    }

    func fireIfExpired() {
        guard let lastWord, now() >= lastWord + interval else { return }
        cancel()
        onExpired?()
    }

    private func schedule() {
        task?.cancel()
        guard let lastWord else { return }
        let delay = max(0, lastWord + interval - now())
        task = Task { [weak self] in
            do { try await Task.sleep(for: .seconds(delay)) } catch { return }
            guard !Task.isCancelled else { return }
            self?.fireIfExpired()
        }
    }
}