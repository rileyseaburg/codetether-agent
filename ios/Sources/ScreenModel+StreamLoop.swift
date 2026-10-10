import Foundation

extension ScreenModel {
    /// Follow one owned session; reconnect streams, never replay owner commands.
    func followStream(_ id: UUID, ticket: UUID) async {
        let retry = ScreenStreamRetry(sequence: response.sequence)
        while true {
            guard !Task.isCancelled, generation == ticket, active else { return }
            do {
                try await client.events(id, token: credential()) { [weak self] event in
                    guard let self, generation == ticket, !Task.isCancelled else { return }
                    retry.receive(event)
                    receiveScreenEvent(event)
                }
                if session == nil { return }
                throw ScreenFailure.disconnected
            } catch {
                guard !Task.isCancelled, generation == ticket else { return }
                connected = false; self.error = ScreenFailure.message(error)
                if case ScreenFailure.http(let code) = error, code == 404 || code == 410 {
                    finishSession("This screen session ended. Create a new session to pair again.")
                    return
                }
                guard let wait = retry.nextDelay(after: error) else {
                    retryBlocked = true; notice = "Stream paused. Reconnect or Stop."; return
                }
                notice = "Reconnecting to live analysis…"
                do { try await delay(wait) }
                catch { return }
            }
        }
    }
}
