//! Keyboard capture provider trait and feature flags.

use crate::overlay::InputEvent;
use anyhow::Result;
use std::sync::Arc;

/// Feature flags describing what a capture provider supports.
#[derive(Debug, Clone, Default)]
pub struct CaptureFeatures {
    pub keyboard: bool,
    pub mouse: bool,
    pub scroll: bool,
    pub gamepad: bool,
    /// Provider can detect which application has focus.
    pub app_context: bool,
}

/// Platform-specific keyboard capture provider.
///
/// Implementations read raw input events from the platform and broadcast
/// them for processing. The provider owns the capture lifecycle.
///
/// # Platform Implementations
///
/// - **Linux:** `EvdevCapture` reads from `/dev/input/event*` via evdev
/// - **Windows:** `WindowsCapture` uses `SetWindowsHookEx`
/// - **macOS:** `MacosCapture` uses `CGEventTap`
#[async_trait::async_trait]
pub trait KeyboardCaptureProvider: Send + Sync {
    /// Start capturing keyboard events.
    ///
    /// After this returns, events will be available via `subscribe()`.
    async fn start(&mut self) -> Result<()>;

    /// Stop capturing keyboard events.
    async fn stop(&mut self) -> Result<()>;

    /// Subscribe to input events.
    ///
    /// Returns a broadcast receiver. Each subscriber gets its own copy
    /// of every event. Use `broadcast::Receiver::resubscribe()` for
    /// multiple consumers.
    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<InputEvent>;

    /// Report which features this provider supports.
    fn features(&self) -> CaptureFeatures;

    /// Provider name for logging/debugging.
    fn name(&self) -> &str;
}

/// Type alias for a shared, boxed capture provider.
pub type SharedCapture = Arc<dyn KeyboardCaptureProvider>;

/// Factory for creating platform-specific keyboard capture providers.
///
/// Each platform provides its own factory implementation:
/// - **Linux:** `EvdevCaptureFactory`
/// - **Windows:** `WindowsCaptureFactory`
/// - **macOS:** `MacosCaptureFactory`
pub trait KeyboardCaptureFactory: Send + Sync {
    /// Create a new capture provider for this platform.
    fn create(&self) -> Box<dyn KeyboardCaptureProvider>;

    /// Platform name for logging.
    fn platform_name(&self) -> &str;
}

/// Type alias for a shared, boxed capture factory.
pub type SharedCaptureFactory = Arc<dyn KeyboardCaptureFactory>;