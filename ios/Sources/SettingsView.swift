import SwiftUI

struct SettingsView: View {
    @ObservedObject var model: ConnectionModel
    @Environment(\.dismiss) private var dismiss
    @State private var token = ""
    @State private var error: String?

    var body: some View {
        NavigationStack {
            Form {
                Section("Server") { Text(ServerClient.origin.absoluteString).font(.footnote) }
                Section {
                    SecureField("Bearer token", text: $token)
                        .textInputAutocapitalization(.never).autocorrectionDisabled()
                        .privacySensitive()
                    Button("Save & connect") { save() }
                        .disabled(token.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)
                } header: { Text("Authentication") } footer: {
                    Text("Stored only in this iPhone’s Keychain. Never included in the app or synced to iCloud.")
                }
                if let error { Text(error).foregroundStyle(.red) }
                Button("Remove saved token", role: .destructive) {
                    do { try model.disconnect(); token = ""; dismiss() }
                    catch { self.error = error.localizedDescription }
                }
            }
            .navigationTitle("Settings")
            .toolbar { Button("Done") { dismiss() } }
        }
    }

    private func save() {
        let clean = token.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !clean.isEmpty, clean.rangeOfCharacter(from: .controlCharacters) == nil else {
            error = "Enter a single bearer token."; return
        }
        do {
            try TokenStore.save(clean)
            token = ""
            dismiss()
            Task { await model.refresh() }
        } catch { self.error = error.localizedDescription }
    }
}
