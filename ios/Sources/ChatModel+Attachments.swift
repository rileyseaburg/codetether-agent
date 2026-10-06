import UIKit

extension ChatModel {
    func addAttachment(_ image: UserImage) {
        guard !busy, attachments.count < UserImage.maximumAttachments else { return }
        attachments.append(image)
    }

    func attachCapture(_ image: UIImage?) {
        guard let image, !busy, attachments.count < UserImage.maximumAttachments else { return }
        do {
            addAttachment(try UserImage.prepare(image))
        } catch {
            self.error = "That photo could not be attached. Try another photo."
        }
    }
}