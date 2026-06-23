use crate::keys::VirtualKey;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// State of a key press.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyState {
    Pressed,
    Released,
}

/// Modifier key state bitmask.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModifierState {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub super_key: bool,
    /// CapsLock toggle state (on/off)
    pub capslock: bool,
    /// NumLock toggle state (on/off)
    pub numlock: bool,
}

impl ModifierState {
    /// Returns true if no modifiers are active.
    pub fn is_empty(&self) -> bool {
        !self.ctrl && !self.alt && !self.shift && !self.super_key
    }

    /// Returns the number of active modifier keys.
    pub fn count(&self) -> u8 {
        [self.ctrl, self.alt, self.shift, self.super_key]
            .iter()
            .filter(|&&b| b)
            .count() as u8
    }

    /// Build display prefix like "⌃⌥⇧⌘" for standard mode.
    pub fn symbol_prefix(&self) -> String {
        let mut s = String::new();
        if self.ctrl {
            s.push_str("⌃ ");
        }
        if self.alt {
            s.push_str("⌥ ");
        }
        if self.shift {
            s.push_str("⇧ ");
        }
        if self.super_key {
            s.push_str("⌘ ");
        }
        s
    }

    /// Returns true if CapsLock is currently enabled.
    pub fn is_capslock_on(&self) -> bool {
        self.capslock
    }

    /// Returns true if NumLock is currently enabled.
    pub fn is_numlock_on(&self) -> bool {
        self.numlock
    }
}

impl std::fmt::Display for ModifierState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();
        if self.ctrl {
            parts.push("Ctrl");
        }
        if self.alt {
            parts.push("Alt");
        }
        if self.shift {
            parts.push("Shift");
        }
        if self.super_key {
            parts.push("Super");
        }
        write!(f, "{}", parts.join(" + "))
    }
}

/// A raw keyboard event from the capture provider.
#[derive(Debug, Clone)]
pub struct KeyboardEvent {
    pub key: VirtualKey,
    pub state: KeyState,
    pub timestamp: SystemTime,
    /// Raw platform scancode (evdev code on Linux, virtual key code on Windows).
    pub native_code: u32,
}

/// A grouped keyboard shortcut ready for display.
///
/// This represents a completed shortcut like "Ctrl + Shift + P"
/// rather than individual key events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShortcutCombo {
    pub modifiers: ModifierState,
    pub key: Option<VirtualKey>,
    /// Human-readable display string: "Ctrl + Shift + P"
    pub display: String,
    /// Sequence of plain keystrokes (no modifiers), displayed horizontally.
    /// When non-empty, this combo represents multiple sequential key presses.
    #[serde(default)]
    pub key_sequence: Vec<VirtualKey>,
    /// Pre-resolved display text for character events.
    /// When set, the renderer shows this instead of building from modifiers+key.
    /// Examples: "a", "A", "!", "€"
    #[serde(default)]
    pub resolved_text: Option<String>,
    /// Resolved characters for each key in a sequence.
    /// Parallel to `key_sequence` — same length, one resolved char per key.
    /// When non-empty, sequence rendering uses these instead of `key.label()`.
    #[serde(default)]
    pub resolved_chars: Vec<String>,
}

impl ShortcutCombo {
    /// Create a new shortcut combo from modifiers and key.
    pub fn new(modifiers: ModifierState, key: Option<VirtualKey>) -> Self {
        let display = Self::build_display(&modifiers, key.as_ref());
        Self {
            modifiers,
            key,
            display,
            key_sequence: Vec::new(),
            resolved_text: None,
            resolved_chars: Vec::new(),
        }
    }

    /// Create a key sequence combo from multiple keys (no modifiers).
    pub fn sequence(keys: Vec<VirtualKey>) -> Self {
        let display: String = keys.iter().map(|k| k.label()).collect::<Vec<_>>().join(" ");
        Self {
            modifiers: ModifierState::default(),
            key: keys.last().copied(),
            display,
            key_sequence: keys,
            resolved_text: None,
            resolved_chars: Vec::new(),
        }
    }

    /// Create a character combo — a single resolved symbol.
    pub fn character(key: VirtualKey, resolved_text: String) -> Self {
        Self {
            modifiers: ModifierState::default(),
            key: Some(key),
            display: resolved_text.clone(),
            key_sequence: Vec::new(),
            resolved_text: Some(resolved_text),
            resolved_chars: Vec::new(),
        }
    }

    /// Create a sequence from resolved characters (for character merging).
    pub fn resolved_sequence(keys: Vec<VirtualKey>, chars: Vec<String>) -> Self {
        let display = chars.join("");
        Self {
            modifiers: ModifierState::default(),
            key: keys.last().copied(),
            display,
            key_sequence: keys,
            resolved_text: None,
            resolved_chars: chars,
        }
    }

    /// Returns true if this combo represents a plain keystroke sequence.
    pub fn is_sequence(&self) -> bool {
        !self.key_sequence.is_empty()
    }

    fn build_display(mods: &ModifierState, key: Option<&VirtualKey>) -> String {
        let mut parts = Vec::new();
        if mods.ctrl {
            parts.push("Ctrl".to_string());
        }
        if mods.alt {
            parts.push("Alt".to_string());
        }
        if mods.shift {
            parts.push("Shift".to_string());
        }
        if mods.super_key {
            parts.push("Super".to_string());
        }
        if let Some(k) = key {
            let key_label = Self::get_key_display(mods, k);
            parts.push(key_label);
        }
        parts.join(" + ")
    }

    /// Get the display label for a key given the current modifier state.
    /// Handles shifted symbols, CapsLock for letters, and NumLock for numpad.
    pub fn get_key_display(mods: &ModifierState, key: &VirtualKey) -> String {
        // Check for shifted symbols first
        if mods.shift {
            if let Some(shifted) = key.shifted_label() {
                return shifted;
            }
        }

        // Check for numpad keys with NumLock off
        if !mods.is_numlock_on() {
            if let Some(numlock_off) = key.numlock_off_label() {
                return numlock_off;
            }
        }

        // Handle letters with CapsLock
        // Only apply CapsLock logic when no other modifiers (Ctrl, Alt, Super) are held.
        // In shortcut combinations like Ctrl+C, show the key label as uppercase.
        if key.is_letter() && !mods.ctrl && !mods.alt && !mods.super_key {
            let base = key.label();
            let is_uppercase = mods.is_capslock_on() ^ mods.shift; // XOR: Shift reverses CapsLock
            if is_uppercase {
                base.to_uppercase()
            } else {
                base.to_lowercase()
            }
        } else {
            key.label()
        }
    }

    /// Returns true if this is a modifier-only combo (no non-modifier key).
    pub fn is_modifier_only(&self) -> bool {
        self.key.is_none()
    }
}

impl std::fmt::Display for ShortcutCombo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}

/// Mouse button identifier for future mouse support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Unknown(u32),
}

/// A mouse event for future mouse visualization support.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MouseEvent {
    Button {
        button: MouseButton,
        state: KeyState,
        timestamp: SystemTime,
    },
    Scroll {
        direction: ScrollDirection,
        amount: i32,
        timestamp: SystemTime,
    },
}

/// Scroll direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Unified input event from any capture provider.
#[derive(Debug, Clone)]
pub enum InputEvent {
    Keyboard(KeyboardEvent),
    Mouse(MouseEvent),
}

/// A processed event ready for overlay display.
#[derive(Debug, Clone)]
pub enum ProcessedEvent {
    /// A completed shortcut combo (grouped mode).
    Shortcut(ShortcutCombo),
    /// A resolved character to display as a single keycap.
    Character(String),
    /// Modifier state changed (for tracking held modifiers).
    ModifierChange(ModifierState),
    /// A single key event (raw mode).
    RawKey(KeyboardEvent),
}

/// Application context info for future app-aware shortcuts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppContext {
    /// Application name (e.g., "Code", "Firefox").
    pub name: String,
    /// Window title if available.
    pub title: Option<String>,
    /// Desktop entry / WM_CLASS if available.
    pub class: Option<String>,
}

/// An enriched shortcut with optional app context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichedShortcut {
    pub shortcut: ShortcutCombo,
    pub app_context: Option<AppContext>,
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keys::VirtualKey;

    #[test]
    fn test_shifted_symbols_display() {
        // Test all shifted symbols show resulting character
        let mods = ModifierState {
            shift: true,
            ..Default::default()
        };

        // Number row symbols
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::Key1)),
            "Shift + !"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::Key2)),
            "Shift + @"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::Key3)),
            "Shift + #"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::Key4)),
            "Shift + $"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::Key5)),
            "Shift + %"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::Key6)),
            "Shift + ^"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::Key7)),
            "Shift + &"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::Key8)),
            "Shift + *"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::Key9)),
            "Shift + ("
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::Key0)),
            "Shift + )"
        );

        // Punctuation & symbols
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::Minus)),
            "Shift + _"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::Equal)),
            "Shift + +"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::LeftBracket)),
            "Shift + {"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::RightBracket)),
            "Shift + }"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::Backslash)),
            "Shift + |"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::Semicolon)),
            "Shift + :"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::Quote)),
            "Shift + \""
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::Comma)),
            "Shift + <"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::Period)),
            "Shift + >"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::Slash)),
            "Shift + ?"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods, Some(&VirtualKey::Backtick)),
            "Shift + ~"
        );
    }

    #[test]
    fn test_capslock_letters_display() {
        // CapsLock off, no Shift -> lowercase
        let mods_off = ModifierState {
            capslock: false,
            shift: false,
            ..Default::default()
        };
        assert_eq!(
            ShortcutCombo::build_display(&mods_off, Some(&VirtualKey::A)),
            "a"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_off, Some(&VirtualKey::Z)),
            "z"
        );

        // CapsLock on, no Shift -> uppercase
        let mods_on = ModifierState {
            capslock: true,
            shift: false,
            ..Default::default()
        };
        assert_eq!(
            ShortcutCombo::build_display(&mods_on, Some(&VirtualKey::A)),
            "A"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_on, Some(&VirtualKey::Z)),
            "Z"
        );

        // CapsLock off, Shift held -> uppercase
        let mods_shift = ModifierState {
            capslock: false,
            shift: true,
            ..Default::default()
        };
        assert_eq!(
            ShortcutCombo::build_display(&mods_shift, Some(&VirtualKey::A)),
            "Shift + A"
        );

        // CapsLock on, Shift held -> lowercase (Shift reverses CapsLock)
        let mods_both = ModifierState {
            capslock: true,
            shift: true,
            ..Default::default()
        };
        assert_eq!(
            ShortcutCombo::build_display(&mods_both, Some(&VirtualKey::A)),
            "Shift + a"
        );
    }

    #[test]
    fn test_capslock_with_other_modifiers_shows_uppercase() {
        // Ctrl + C should show uppercase C, not lowercase
        let mods_ctrl = ModifierState {
            ctrl: true,
            capslock: false,
            shift: false,
            ..Default::default()
        };
        assert_eq!(
            ShortcutCombo::build_display(&mods_ctrl, Some(&VirtualKey::C)),
            "Ctrl + C"
        );

        // Ctrl + Shift + C should show uppercase C
        let mods_ctrl_shift = ModifierState {
            ctrl: true,
            shift: true,
            capslock: false,
            ..Default::default()
        };
        assert_eq!(
            ShortcutCombo::build_display(&mods_ctrl_shift, Some(&VirtualKey::C)),
            "Ctrl + Shift + C"
        );
    }

    #[test]
    fn test_numlock_on_numpad_display() {
        let mods_on = ModifierState {
            numlock: true,
            ..Default::default()
        };

        assert_eq!(
            ShortcutCombo::build_display(&mods_on, Some(&VirtualKey::Numpad0)),
            "Num0"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_on, Some(&VirtualKey::Numpad5)),
            "Num5"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_on, Some(&VirtualKey::Numpad9)),
            "Num9"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_on, Some(&VirtualKey::NumpadAdd)),
            "Num+"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_on, Some(&VirtualKey::NumpadSubtract)),
            "Num-"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_on, Some(&VirtualKey::NumpadMultiply)),
            "Num*"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_on, Some(&VirtualKey::NumpadDivide)),
            "Num/"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_on, Some(&VirtualKey::NumpadDecimal)),
            "Num."
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_on, Some(&VirtualKey::NumpadEnter)),
            "NumEnter"
        );
    }

    #[test]
    fn test_numlock_off_numpad_display() {
        let mods_off = ModifierState {
            numlock: false,
            ..Default::default()
        };

        assert_eq!(
            ShortcutCombo::build_display(&mods_off, Some(&VirtualKey::Numpad0)),
            "Ins"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_off, Some(&VirtualKey::Numpad1)),
            "End"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_off, Some(&VirtualKey::Numpad2)),
            "Down"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_off, Some(&VirtualKey::Numpad3)),
            "PgDn"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_off, Some(&VirtualKey::Numpad4)),
            "Left"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_off, Some(&VirtualKey::Numpad5)),
            "Num5"
        ); // No nav equivalent
        assert_eq!(
            ShortcutCombo::build_display(&mods_off, Some(&VirtualKey::Numpad6)),
            "Right"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_off, Some(&VirtualKey::Numpad7)),
            "Home"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_off, Some(&VirtualKey::Numpad8)),
            "Up"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_off, Some(&VirtualKey::Numpad9)),
            "PgUp"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_off, Some(&VirtualKey::NumpadDecimal)),
            "Del"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_off, Some(&VirtualKey::NumpadEnter)),
            "Enter"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_off, Some(&VirtualKey::NumpadAdd)),
            "Num+"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_off, Some(&VirtualKey::NumpadSubtract)),
            "Num-"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_off, Some(&VirtualKey::NumpadMultiply)),
            "Num*"
        );
        assert_eq!(
            ShortcutCombo::build_display(&mods_off, Some(&VirtualKey::NumpadDivide)),
            "Num/"
        );
    }

    #[test]
    fn test_letter_display_with_capslock_and_shift_combinations() {
        // All combinations for a letter key
        let test_cases = vec![
            // (capslock, shift, expected)
            (false, false, "a"),
            (true, false, "A"),
            (false, true, "Shift + A"),
            (true, true, "Shift + a"),
        ];

        for (capslock, shift, expected) in test_cases {
            let mods = ModifierState {
                capslock,
                shift,
                ..Default::default()
            };
            let result = ShortcutCombo::build_display(&mods, Some(&VirtualKey::A));
            assert_eq!(
                result, expected,
                "Failed for capslock={}, shift={}",
                capslock, shift
            );
        }
    }
}
