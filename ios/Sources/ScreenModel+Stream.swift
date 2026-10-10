import Foundation

extension ScreenModel {
    func setActive(_ value: Bool) {
        guard active != value else { return }
        active = value
        if value { connect() }
        else if !creating && !stopping { invalidateStream() }
    }
    func reconnect() {
        guard canReconnect else { return }
        retryBlocked = false; connect()
    }
    func connect() {
        guard active, !creating, !stopping, !retryBlocked, let owned = session else { return }
        let previous = streamTask
        invalidateStream()
        let ticket = generation
        streamTask = Task { [weak self] in
            await previous?.value
            guard let self else { return }
            await followStream(owned.id, ticket: ticket)
        }
    }
}
