import SwiftUI

struct DashboardView: View {
    @ObservedObject var model: ConnectionModel
    @State private var showSettings = false

    var body: some View {
        NavigationStack {
            List {
                Section {
                    Label(model.version == nil ? "Not connected" : "Authenticated",
                          systemImage: model.version == nil ? "network.slash" : "checkmark.shield.fill")
                        .foregroundStyle(model.version == nil ? Color.secondary : Color.green)
                    Text("server.codetether.run").font(.footnote).textSelection(.enabled)
                    if let version = model.version {
                        LabeledContent("Server version", value: version.version)
                    }
                    if let date = model.checkedAt {
                        LabeledContent("Last checked", value: date.formatted(date: .omitted, time: .standard))
                    }
                    if model.busy { ProgressView("Connecting securely…") }
                    if let error = model.error { Text(error).foregroundStyle(.red) }
                } header: { Text("Connection") }
                Section("Agents · \(model.agents.count)") {
                    ForEach(model.agents) { agent in
                        VStack(alignment: .leading, spacing: 6) {
                            Text(agent.name).font(.headline)
                            Text(agent.mode.capitalized).font(.caption).foregroundStyle(.tint)
                            if let description = agent.description {
                                Text(description).font(.subheadline).foregroundStyle(.secondary)
                            }
                        }.padding(.vertical, 4)
                    }
                }
            }
            .navigationTitle("CodeTether")
            .refreshable { await model.refresh() }
            .toolbar {
                Button("Refresh", systemImage: "arrow.clockwise") { Task { await model.refresh() } }
                    .disabled(model.busy)
                Button("Settings", systemImage: "gearshape") { showSettings = true }.disabled(model.busy)
            }
            .sheet(isPresented: $showSettings) { SettingsView(model: model) }
        }
    }
}
