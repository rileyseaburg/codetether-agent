import Foundation

extension ScreenModel {
    func setActive(_ value: Bool) {
        active = value
        if value { connect() }
        else if !creating && !stopping { invalidateStream() }
    }
    func reconnect() { retryBlocked = false; invalidateStream(); connect() }
    func connect() {
        guard active, !creating, !stopping, !retryBlocked, let owned = session else { return }
        let previous = streamTask
        invalidateStream()
        let ticket = generation
        streamTask = Task { [weak self] in
            await previous?.value
            guard let self else { return }
            for attempt in 0..<5 {
                guard !Task.isCancelled, generation == ticket, active else { return }
                do {
                    try await client.events(owned.id, token: credential()) { [weak self] event in
                        guard let self, generation == ticket, !Task.isCancelled else { return }
                        response.apply(event)
                        connected = true; error = nil; notice = "Live screen analysis"
                        if event.type == .stopped {
                            session = nil; connected = false; notice = "Session stopped."
                        }
                    }
                    if session == nil { return }
                    throw ScreenFailure.disconnected
                } catch {
                    guard !Task.isCancelled, generation == ticket else { return }
                    connected = false; self.error = ScreenFailure.message(error)
                    guard ScreenFailure.retryable(error), attempt < 4 else {
                        retryBlocked = true; notice = "Stream paused. Reconnect or Stop."; return
                    }
                    notice = "Reconnecting; Windows uploads are paused."
                    do { try await delay(UInt64(1 << attempt) * 1_000_000_000) }
                    catch { return }
                }
            }
        }
    }
}