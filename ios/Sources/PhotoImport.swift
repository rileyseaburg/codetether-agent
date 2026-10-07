import Foundation
import PhotosUI
import SwiftUI

/// Loads photos chosen in the picker into the chat as attachments.
enum PhotoImport {
    @MainActor
    static func handle(_ items: [PhotosPickerItem], chat: ChatModel) async {
        for item in items.prefix(max(0, UserImage.maximumAttachments - chat.attachments.count)) {
            do {
                if let data = try await item.loadTransferable(type: Data.self) {
                    chat.addAttachment(try UserImage.prepare(data))
                }
            } catch { chat.error = "That image could not be attached. Try another photo." }
        }
    }
}
