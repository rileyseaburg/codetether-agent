import SwiftUI

struct ScreenResponseView: View {
    @ObservedObject var model: ScreenModel
    var body: some View {
        Section("Live analysis") {
            Label(model.connected ? model.response.status : model.notice,
                  systemImage: model.connected ? "dot.radiowaves.left.and.right" : "pause.circle")
                .foregroundStyle(model.connected ? .green : .secondary)
            if let captured = model.response.capturedAt, let date = ScreenDate.parse(captured) {
                Text("Latest capture: \(date.formatted())").font(.caption).foregroundStyle(.secondary)
            }
            Text(model.response.text.isEmpty ? "Waiting for a screenshot from Windows…" : model.response.text)
                .textSelection(.enabled).accessibilityIdentifier("screen-analysis")
            if !model.connected && !model.stopping {
                Button("Reconnect stream") { model.reconnect() }
                    .accessibilityIdentifier("screen-reconnect")
            }
            Button(model.stopping ? "Stopping…" : "Stop and revoke pairing", role: .destructive) {
                Task { await model.stop() }
            }
            .disabled(model.stopping).accessibilityIdentifier("screen-stop")
        }
    }
}