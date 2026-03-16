use super::ExtEvent;
use std::sync::Arc;
use xframe::{XFrameProxy, winit::event_loop::EventLoopClosed};

/// Proxy to send callbacks to the main application implementation.
///
/// The proxy can be cloned.
#[derive(Clone)]
pub struct UiEventProxy(Arc<XFrameProxy<ExtEvent>>);
impl UiEventProxy {
    pub fn new(proxy: Arc<XFrameProxy<ExtEvent>>) -> Self {
        Self(proxy)
    }

    /// Sends the given `event` to the ui-event-loop (`winit::event_loop`) thread
    /// and can be received on the main application XFrameApp implementation.
    ///
    /// Returns `EventLoopClosed` error if the associated EventLoop no longer exists.
    pub fn send(&self, ui_event: ExtEvent) -> Result<(), EventLoopClosed<()>> {
        if self.0.send_custom_event(ui_event).is_err() {
            return Err(EventLoopClosed(()));
        }

        Ok(())
    }
}
