//! Core traits.

use crate::events::{InputEvent, ProcessedEvent, ModifierState, ShortcutCombo};
use anyhow::Result;

/// Platform-independent event processor.
///
/// Consumes raw `InputEvent`s and produces `ProcessedEvent`s ready
/// for the overlay. Manages modifier state, event grouping, and
/// history.
pub trait EventProcessor: Send + Sync {
    /// Process a raw input event.
    ///
    /// Returns zero or more display-ready events. For example, a key
    /// release might complete a shortcut combo and produce a
    /// `ProcessedEvent::Shortcut`.
    fn process(&mut self, event: InputEvent) -> Vec<ProcessedEvent>;

    /// Get current modifier state.
    fn modifier_state(&self) -> ModifierState;

    /// Get the shortcut currently being composed (held modifiers + key).
    fn current_compose(&self) -> Option<ShortcutCombo>;

    /// Get recent shortcut history (most recent first).
    fn history(&self) -> &[ShortcutCombo];

    /// Clear history.
    fn clear_history(&mut self);

    /// Update processor configuration.
    fn update_config(&mut self, config: ProcessorConfig);
}

/// Configuration for the event processor.
#[derive(Debug, Clone)]
pub struct ProcessorConfig {
    /// Maximum number of shortcuts to keep in history.
    pub history_length: usize,
    /// Whether to group modifier+key combos into single shortcuts.
    pub group_shortcuts: bool,
    /// Minimum time between duplicate events (deduplication).
    pub dedup_window: std::time::Duration,
}

impl Default for ProcessorConfig {
    fn default() -> Self {
        Self {
            history_length: 10,
            group_shortcuts: true,
            dedup_window: std::time::Duration::from_millis(50),
        }
    }
}