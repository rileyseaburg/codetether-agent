import SwiftUI

struct ScreenPairingView: View {
    let session: ScreenSession
    let status: String
    var body: some View {
        Section("Windows companion") {
            Text("On Windows, open Edge or Chrome:")
            Text("https://server.codetether.run/companion/")
                .font(.footnote).textSelection(.enabled)
            if status == "waiting" || status == "Waiting for Windows pairing" {
                Text(session.code).font(.title2.monospaced().bold())
                    .textSelection(.enabled).accessibilityIdentifier("screen-pair-code")
                if let expiry = session.pairingExpiry {
                    Text("One-use code expires \(expiry.formatted(date: .omitted, time: .shortened)).")
                        .font(.caption).foregroundStyle(.secondary)
                }
            } else {
                Label("Windows paired", systemImage: "link")
            }
            Text("Enter the code there, then choose a window and press Start. Installing the companion is optional: use Edge Apps → Install this site as an app.")
                .font(.footnote).foregroundStyle(.secondary)
        }
    }
}