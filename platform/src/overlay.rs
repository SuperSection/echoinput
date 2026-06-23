//! Overlay renderer trait and configuration types (re-exported from input-core).

pub use input_core::events::InputEvent;
pub use input_core::overlay::{
    AnimationType, BackgroundSettings, BorderSettings, ColorSettings, DisplayEvent, KeycapStyle,
    OverlayConfig, OverlayPosition, OverlayScale, TextCaps, TextSettings, TextVariant, Theme,
};

use anyhow::Result;

/// Cross-platform overlay renderer.
///
/// Each platform provides its own renderer:
/// - **Linux Wayland:** Layer-shell surface with Cairo/EGL rendering
/// - **Linux X11:** Transparent always-on-top window
/// - **Windows:** Transparent layered window
/// - **macOS:** NSPanel with panel level
#[async_trait::async_trait]
pub trait OverlayRenderer: Send + Sync {
    /// Initialize the overlay with configuration.
    async fn start(&mut self, config: OverlayConfig) -> Result<()>;

    /// Tear down the overlay.
    async fn stop(&mut self) -> Result<()>;

    /// Update the overlay display content.
    fn update(&self, event: DisplayEvent) -> Result<()>;

    /// Check if the overlay is currently running.
    fn is_running(&self) -> bool;

    /// Renderer name for logging.
    fn name(&self) -> &str;
}

/// Factory for creating platform-specific overlay renderers.
///
/// Each platform provides its own factory implementation:
/// - **Linux Wayland:** `WaylandRendererFactory`
/// - **Linux X11:** `X11RendererFactory`
/// - **Windows:** `WindowsRendererFactory`
/// - **macOS:** `MacRendererFactory`
pub trait OverlayRendererFactory: Send + Sync {
    /// Create a new renderer for this platform.
    fn create(&self, bus: input_core::ipc::MessageBus) -> Box<dyn OverlayRenderer>;

    /// Platform name for logging.
    fn platform_name(&self) -> &str;
}
