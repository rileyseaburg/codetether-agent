import UIKit

extension TranscriptScrollController {
    func scheduleScroll(after delay: TimeInterval = 0) {
        guard !scheduled else { return }
        scheduled = true
        // Wait for SwiftUI/UIKit to finish the layout which changed contentSize/bounds.
        DispatchQueue.main.asyncAfter(deadline: .now() + delay) { [weak self] in
            guard let self else { return }
            self.scheduled = false
            guard self.following, let view = self.scrollView else { return }
            if view.isTracking || view.isDecelerating {
                self.scheduleScroll(after: 0.05)
                return
            }
            self.scrollToBottom(view)
        }
    }

    func resume() {
        following = true
        if let view = scrollView { scrollToBottom(view) }
        scheduleScroll()
    }

    private func scrollToBottom(_ view: UIScrollView) {
        let target = TranscriptScrollGeometry.bottomOffset(view)
        guard abs(view.contentOffset.y - target) > 0.5 else { return }
        UIView.performWithoutAnimation {
            view.setContentOffset(CGPoint(x: view.contentOffset.x, y: target), animated: false)
        }
    }

    @objc func handlePan(_ gesture: UIPanGestureRecognizer) {
        guard let view = scrollView else { return }
        if gesture.state == .began { following = false }
        if gesture.state == .ended || gesture.state == .cancelled {
            following = TranscriptScrollGeometry.isNearBottom(view)
            scheduleScroll()
        }
    }
}