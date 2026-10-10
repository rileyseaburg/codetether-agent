import Foundation

extension ScreenModel {
    /// Expiration does not depend on a viewer stream being connected.
    func scheduleExpiry(for owned: ScreenSession) {
        expiryTask?.cancel()
        guard let expiry = owned.expiry else { return }
        expiryTask = Task { [weak self] in
            while !Task.isCancelled {
                let remaining = expiry.timeIntervalSinceNow
                if remaining <= 0 { break }
                do { try await Task.sleep(nanoseconds: UInt64(min(remaining, 3600) * 1_000_000_000)) }
                catch { return }
            }
            guard let self, !Task.isCancelled, session?.id == owned.id else { return }
            finishSession("This screen session expired. Create a new session and pair Windows again.")
        }
    }
}
