import SwiftUI

/// The catalog is not capability metadata; the user must select a vision model.
struct ScreenModelPicker: View {
    @ObservedObject var selection: ScreenModelSelection
    @Environment(\.dismiss) private var dismiss
    @State private var search = ""

    var body: some View {
        NavigationStack {
            List {
                Section {
                    Text("Choose a model that accepts images. The server catalog also includes text-only models.")
                        .font(.footnote).foregroundStyle(.secondary)
                    if selection.loading { ProgressView("Loading models…") }
                    if let error = selection.error { Text(error).foregroundStyle(.red) }
                    Button("Reload models") { Task { await selection.load() } }
                        .disabled(selection.loading)
                }
                ForEach(selection.models.filter {
                    search.isEmpty || $0.localizedCaseInsensitiveContains(search)
                }, id: \.self) { model in
                    Button {
                        selection.selected = model
                        dismiss()
                    } label: {
                        HStack {
                            Text(model).font(.subheadline).foregroundStyle(.primary)
                            Spacer()
                            if selection.selected == model { Image(systemName: "checkmark") }
                        }
                    }
                }
            }
            .navigationTitle("Screen vision model")
            .searchable(text: $search)
            .toolbar { Button("Done") { dismiss() } }
        }
    }
}
