import SwiftUI

struct ModelPickerView: View {
    @ObservedObject var chat: ChatModel
    @Environment(\.dismiss) private var dismiss
    @State private var search = ""
    var body: some View {
        NavigationStack {
            List(chat.models.filter { search.isEmpty || $0.localizedCaseInsensitiveContains(search) }, id: \.self) { model in
                Button {
                    chat.selectedModel = model
                    dismiss()
                } label: {
                    HStack {
                        Text(model).font(.subheadline).foregroundStyle(.primary)
                        Spacer()
                        if model == chat.selectedModel { Image(systemName: "checkmark") }
                }.accessibilityIdentifier("model-option-\(model)")
                }
            }
            .navigationTitle("Choose model").searchable(text: $search)
            .toolbar { Button("Done") { dismiss() } }
        }
    }
}
