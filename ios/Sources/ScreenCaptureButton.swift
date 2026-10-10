import SwiftUI

/// One-tap fresh screenshot using the session instructions; Windows must already be sharing.
struct ScreenCaptureButton: View {
    @ObservedObject var model: ScreenModel
    @ObservedObject var questions: ScreenQuestionState

    var body: some View {
        Section {
            Button {
                model.captureNow()
            } label: {
                Label(questions.submitting ? "Requesting…" : "Capture now", systemImage: "camera.viewfinder")
                    .frame(maxWidth: .infinity)
            }
            .buttonStyle(.borderedProminent)
            .disabled(!model.canAskQuestion)
            .accessibilityIdentifier("screen-capture-now")
            Text(hint).font(.caption).foregroundStyle(.secondary)
        }
    }

    private var hint: String {
        if !model.response.isPaired { return "Available after Windows pairs with this session." }
        if !model.connected { return "Reconnect the analysis stream to capture." }
        if questions.waitingForFrame || ["requested", "analyzing"].contains(model.response.status) {
            return "Waiting for the current capture to finish."
        }
        return "Asks Windows for one fresh screenshot of the shared monitor."
    }
}
