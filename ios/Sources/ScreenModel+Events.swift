import Foundation

extension ScreenModel {
    func receiveScreenEvent(_ event: ScreenEvent) {
        guard event.type == .snapshot || event.seq > response.sequence else { return }
        response.apply(event)
        questions.receive(event)
        connected = true
        error = nil
        notice = "Receiving screen assistance. Explicit typing requests go directly to Windows."
        if event.type == .stopped { finishSession("Session ended on the server.") }
    }
}
