import SwiftUI
import PhotosUI

struct AttachmentBar: View {
    @ObservedObject var chat: ChatModel
    @State private var selection: [PhotosPickerItem] = []
    var body: some View {
        HStack {
            CameraAttachmentButton(chat: chat)
            PhotosPicker(selection: $selection,
                         maxSelectionCount: max(1, UserImage.maximumAttachments - chat.attachments.count), matching: .images) {
                Label("Photos", systemImage: "photo.badge.plus")
            }.accessibilityIdentifier("add-image")
                .disabled(chat.attachments.count >= UserImage.maximumAttachments)
            ScrollView(.horizontal) {
                HStack {
                    ForEach(chat.attachments) { image in
                        if let preview = UIImage(data: image.data) {
                            Image(uiImage: preview).resizable().scaledToFill().frame(width: 56, height: 56).clipped()
                                .overlay(alignment: .topTrailing) {
                                    Button { chat.attachments.removeAll { $0.id == image.id } } label: {
                                        Image(systemName: "xmark.circle.fill").symbolRenderingMode(.palette)
                                            .foregroundStyle(.white, .black)
                                    }.accessibilityLabel("Remove attached image")
                                }
                        }
                    }
                }
            }
        }.frame(height: chat.attachments.isEmpty ? 36 : 64).padding(.horizontal).disabled(chat.busy)
        .onChange(of: selection) { _, items in
            Task {
                for item in items.prefix(max(0, UserImage.maximumAttachments - chat.attachments.count)) {
                    do {
                        if let data = try await item.loadTransferable(type: Data.self) {
                            chat.addAttachment(try UserImage.prepare(data))
                        }
                    } catch { chat.error = "That image could not be attached. Try another photo." }
                }
                selection = []
            }
        }
    }
}