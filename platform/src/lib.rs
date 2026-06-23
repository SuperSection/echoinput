//! Platform abstraction layer for EchoInput.
//!
//! This crate defines the traits and types that platform-specific
//! implementations must provide. It sits between the core logic and
//! platform implementations.

pub mod capture;
pub mod factory;
pub mod overlay;
pub mod processor;

// Re-export key types for convenience
pub use capture::{CaptureFeatures, KeyboardCaptureProvider};
pub use factory::{KeyboardCaptureFactory, OverlayRendererFactory};
pub use overlay::{DisplayEvent, OverlayConfig, OverlayRenderer};
pub use processor::EventProcessor;
