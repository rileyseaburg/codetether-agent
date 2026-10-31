import SwiftUI

/// Scrubbing changes only audio position; the transcript retains its own scroll view.
struct ReadbackControls: View {
    @ObservedObject var transport: ReadbackTransport
    @State private var dragging = false
    @State private var position: TimeInterval = 0
    var body: some View {
        if transport.active {
            VStack(spacing: 8) {
                HStack(spacing: 32) {
                    Button { transport.skip(-15) } label: { Image(systemName: "gobackward.15") }
                        .accessibilityLabel("Rewind 15 seconds").accessibilityIdentifier("readback-rewind")
                    Button { transport.toggle() } label: { Image(systemName: transport.paused ? "play.circle.fill" : "pause.circle.fill") }
                        .accessibilityLabel(transport.paused ? "Play readback" : "Pause readback")
                        .accessibilityIdentifier("readback-play-pause")
                    Button { transport.skip(15) } label: { Image(systemName: "goforward.15") }
                        .accessibilityLabel("Forward 15 seconds").accessibilityIdentifier("readback-forward")
                }.font(.title).buttonStyle(.plain)
                Slider(value: Binding(get: { dragging ? position : transport.elapsed }, set: { position = $0 }),
                       in: 0...max(transport.duration, 0.001), onEditingChanged: { editing in
                    if editing { position = transport.elapsed; dragging = true }
                    else { transport.seek(position); dragging = false }
                }).disabled(transport.duration <= 0)
                    .accessibilityLabel("Readback position").accessibilityIdentifier("readback-position")
                HStack {
                    Text(time(dragging ? position : transport.elapsed)); Spacer()
                    Text(time(transport.duration) + (transport.generating ? " available · preparing more" : ""))
                }.font(.caption.monospacedDigit()).foregroundStyle(.secondary)
            }.padding(.horizontal).padding(.vertical, 8)
        }
    }
    private func time(_ seconds: TimeInterval) -> String {
        let value = Int(max(0, seconds))
        return String(format: "%d:%02d", value / 60, value % 60)
    }
}