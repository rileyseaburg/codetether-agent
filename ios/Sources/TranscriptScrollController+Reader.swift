import UIKit

extension TranscriptScrollController {
    /// KVO can run inside UIKit layout; never publish SwiftUI state from that stack.
    func scheduleReaderUpdate(_ view: UIScrollView) {
        guard !readerUpdateScheduled else { return }
        readerUpdateScheduled = true
        DispatchQueue.main.async { [weak self, weak view] in
            guard let self else { return }
            self.readerUpdateScheduled = false
            guard let view, self.scrollView === view else { return }
            let next = TranscriptScrollGeometry.isNearBottom(view)
            if self.following != next { self.following = next }
        }
    }
}