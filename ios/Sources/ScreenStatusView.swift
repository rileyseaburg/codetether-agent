import SwiftUI

struct ScreenStatusView: View {
    @ObservedObject var model: ScreenModel

    var body: some View {
        Section("Session") {
            Label(model.response.statusTitle,
                  systemImage: model.response.status == "paused" ? "pause.circle" : "desktopcomputer")
            Label(model.connected ? "Live analysis stream" : "Analysis stream disconnected",
                  systemImage: model.connected ? "antenna.radiowaves.left.and.right" : "wifi.slash")
                .font(.subheadline).foregroundStyle(.secondary)
            if let session = model.session, let expiry = session.expiry {
                LabeledContent("Session expires") {
                    Text(expiry, style: .time)
                }.font(.footnote)
            }
            if model.response.status == "paused" {
                Text("Capture is paused. Resume it in the Windows companion; questions cannot bypass local consent.")
                    .font(.footnote).foregroundStyle(.secondary)
            }
            if !model.connected {
                Button("Reconnect") { model.reconnect() }
                    .disabled(!model.canReconnect)
                    .accessibilityIdentifier("screen-reconnect")
            }
        }
    }
}
