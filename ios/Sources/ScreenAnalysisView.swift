import SwiftUI

/// Shows the current result and relay typing status, without a second approval step.
struct ScreenAnalysisView: View {
    let response: ScreenResponse

    var body: some View {
        Section("Screen analysis") {
            if response.status == "analyzing" {
                ProgressView("Analyzing a fresh screenshot…")
            } else if response.status == "requested" {
                ProgressView("Waiting for Windows to capture…")
            }
            if response.text.isEmpty {
                Text("Select a monitor in the paired Windows companion, then use Capture now or Ask.")
                    .foregroundStyle(.secondary)
            } else {
                StableMessageText(text: response.text).equatable()
                    .accessibilityIdentifier("screen-analysis")
            }
            if let captured = response.capturedAt, let date = ScreenDate.parse(captured) {
                Text("Last capture: \(date.formatted(date: .abbreviated, time: .standard))")
                    .font(.caption).foregroundStyle(.secondary)
            }
        }
    }
}
