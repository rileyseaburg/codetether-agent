import SwiftUI

/// A pairing code grants only the separate, locally consented Windows capability.
struct ScreenPairingView: View {
    let session: ScreenSession
    let status: String

    var body: some View {
        Section("Pair Windows") {
            if status == "waiting" {
                TimelineView(.periodic(from: .now, by: 1)) { context in
                    let expired = session.pairingExpiry.map { $0 <= context.date } ?? true
                    if expired {
                        Label("Pairing code expired", systemImage: "clock.badge.exclamationmark")
                        Text("Stop this session and create another to get a new one-use code.")
                            .font(.footnote).foregroundStyle(.secondary)
                    } else {
                        Text(session.code).font(.title2.monospaced().bold())
                            .textSelection(.enabled).privacySensitive()
                            .accessibilityIdentifier("screen-pair-code")
                        if let expiry = session.pairingExpiry {
                            Text("One-use code expires at \(expiry.formatted(date: .omitted, time: .shortened)).")
                                .font(.caption).foregroundStyle(.secondary)
                        }
                    }
                }
                Text("Open CodeTether Companion on Windows and enter this code. Select a monitor, confirm local consent, then press Start on Windows.")
                    .font(.footnote)
                Text("Never enter your iPhone's bearer token on Windows. Pairing alone does not start capture.")
                    .font(.footnote).foregroundStyle(.secondary)
            } else {
                Label("Windows paired", systemImage: "link")
                Text("Only the monitor selected on Windows is shared. Resume or change it locally; the iPhone cannot grant capture permission.")
                    .font(.footnote).foregroundStyle(.secondary)
            }
        }
    }
}
