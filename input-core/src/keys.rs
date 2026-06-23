use serde::{Deserialize, Serialize};
use std::fmt;

/// Platform-independent virtual key representation.
///
/// This enum maps physical key positions to logical names using
/// US QWERTY labels. Display output should use the user's actual
/// keyboard layout via XKB translation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum VirtualKey {
    // Letters
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // Numbers
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,

    // Function keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    // Modifiers
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,
    SuperLeft,
    SuperRight,
    Meta,

    // Navigation & editing
    Tab,
    Escape,
    Space,
    Enter,
    Backspace,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    Up,
    Down,
    Left,
    Right,
    PrintScreen,
    ScrollLock,
    Pause,
    CapsLock,

    // Punctuation & symbols
    Minus,
    Equal,
    LeftBracket,
    RightBracket,
    Backslash,
    Semicolon,
    Quote,
    Comma,
    Period,
    Slash,
    Backtick,

    // Numpad
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadSubtract,
    NumpadMultiply,
    NumpadDivide,
    NumpadDecimal,
    NumpadEnter,
    NumpadLock,

    // Media
    MediaPlay,
    MediaPause,
    MediaStop,
    MediaNext,
    MediaPrev,
    VolumeUp,
    VolumeDown,
    Mute,

    // Browser
    BrowserBack,
    BrowserForward,
    BrowserRefresh,

    // Unknown (carries raw scancode)
    Unknown(u32),
}

impl VirtualKey {
    /// Returns the display label for this key.
    pub fn label(&self) -> String {
        match self {
            Self::A => "A".into(),
            Self::B => "B".into(),
            Self::C => "C".into(),
            Self::D => "D".into(),
            Self::E => "E".into(),
            Self::F => "F".into(),
            Self::G => "G".into(),
            Self::H => "H".into(),
            Self::I => "I".into(),
            Self::J => "J".into(),
            Self::K => "K".into(),
            Self::L => "L".into(),
            Self::M => "M".into(),
            Self::N => "N".into(),
            Self::O => "O".into(),
            Self::P => "P".into(),
            Self::Q => "Q".into(),
            Self::R => "R".into(),
            Self::S => "S".into(),
            Self::T => "T".into(),
            Self::U => "U".into(),
            Self::V => "V".into(),
            Self::W => "W".into(),
            Self::X => "X".into(),
            Self::Y => "Y".into(),
            Self::Z => "Z".into(),

            Self::Key0 => "0".into(),
            Self::Key1 => "1".into(),
            Self::Key2 => "2".into(),
            Self::Key3 => "3".into(),
            Self::Key4 => "4".into(),
            Self::Key5 => "5".into(),
            Self::Key6 => "6".into(),
            Self::Key7 => "7".into(),
            Self::Key8 => "8".into(),
            Self::Key9 => "9".into(),

            Self::F1 => "F1".into(),
            Self::F2 => "F2".into(),
            Self::F3 => "F3".into(),
            Self::F4 => "F4".into(),
            Self::F5 => "F5".into(),
            Self::F6 => "F6".into(),
            Self::F7 => "F7".into(),
            Self::F8 => "F8".into(),
            Self::F9 => "F9".into(),
            Self::F10 => "F10".into(),
            Self::F11 => "F11".into(),
            Self::F12 => "F12".into(),
            Self::F13 => "F13".into(),
            Self::F14 => "F14".into(),
            Self::F15 => "F15".into(),
            Self::F16 => "F16".into(),
            Self::F17 => "F17".into(),
            Self::F18 => "F18".into(),
            Self::F19 => "F19".into(),
            Self::F20 => "F20".into(),
            Self::F21 => "F21".into(),
            Self::F22 => "F22".into(),
            Self::F23 => "F23".into(),
            Self::F24 => "F24".into(),

            Self::ShiftLeft | Self::ShiftRight => "Shift".into(),
            Self::ControlLeft | Self::ControlRight => "Ctrl".into(),
            Self::AltLeft | Self::AltRight => "Alt".into(),
            Self::SuperLeft | Self::SuperRight => "Super".into(),
            Self::Meta => "Meta".into(),

            Self::Tab => "Tab".into(),
            Self::Escape => "Esc".into(),
            Self::Space => "Space".into(),
            Self::Enter => "Enter".into(),
            Self::Backspace => "Backspace".into(),
            Self::Delete => "Del".into(),
            Self::Insert => "Ins".into(),
            Self::Home => "Home".into(),
            Self::End => "End".into(),
            Self::PageUp => "PgUp".into(),
            Self::PageDown => "PgDn".into(),
            Self::Up => "Up".into(),
            Self::Down => "Down".into(),
            Self::Left => "Left".into(),
            Self::Right => "Right".into(),
            Self::PrintScreen => "PrtSc".into(),
            Self::ScrollLock => "ScrLk".into(),
            Self::Pause => "Pause".into(),
            Self::CapsLock => "CapsLk".into(),

            Self::Minus => "-".into(),
            Self::Equal => "=".into(),
            Self::LeftBracket => "[".into(),
            Self::RightBracket => "]".into(),
            Self::Backslash => "\\".into(),
            Self::Semicolon => ";".into(),
            Self::Quote => "'".into(),
            Self::Comma => ",".into(),
            Self::Period => ".".into(),
            Self::Slash => "/".into(),
            Self::Backtick => "`".into(),

            Self::Numpad0 => "Num0".into(),
            Self::Numpad1 => "Num1".into(),
            Self::Numpad2 => "Num2".into(),
            Self::Numpad3 => "Num3".into(),
            Self::Numpad4 => "Num4".into(),
            Self::Numpad5 => "Num5".into(),
            Self::Numpad6 => "Num6".into(),
            Self::Numpad7 => "Num7".into(),
            Self::Numpad8 => "Num8".into(),
            Self::Numpad9 => "Num9".into(),
            Self::NumpadAdd => "Num+".into(),
            Self::NumpadSubtract => "Num-".into(),
            Self::NumpadMultiply => "Num*".into(),
            Self::NumpadDivide => "Num/".into(),
            Self::NumpadDecimal => "Num.".into(),
            Self::NumpadEnter => "NumEnter".into(),
            Self::NumpadLock => "NumLk".into(),

            Self::MediaPlay => "Play".into(),
            Self::MediaPause => "Pause".into(),
            Self::MediaStop => "Stop".into(),
            Self::MediaNext => "Next".into(),
            Self::MediaPrev => "Prev".into(),
            Self::VolumeUp => "Vol+".into(),
            Self::VolumeDown => "Vol-".into(),
            Self::Mute => "Mute".into(),

            Self::BrowserBack => "Back".into(),
            Self::BrowserForward => "Fwd".into(),
            Self::BrowserRefresh => "Refresh".into(),

            Self::Unknown(code) => format!("Key{}", code),
        }
    }

    /// Returns the Linux evdev keycode for this key.
    ///
    /// Returns 0 for keys that don't have a direct evdev mapping
    /// (modifier-only keys, media keys, etc.).
    pub fn evdev_code(&self) -> u32 {
        match self {
            Self::A => 30,
            Self::B => 48,
            Self::C => 46,
            Self::D => 32,
            Self::E => 18,
            Self::F => 33,
            Self::G => 34,
            Self::H => 35,
            Self::I => 23,
            Self::J => 36,
            Self::K => 37,
            Self::L => 38,
            Self::M => 50,
            Self::N => 49,
            Self::O => 24,
            Self::P => 25,
            Self::Q => 16,
            Self::R => 19,
            Self::S => 31,
            Self::T => 20,
            Self::U => 22,
            Self::V => 47,
            Self::W => 17,
            Self::X => 45,
            Self::Y => 21,
            Self::Z => 44,

            Self::Key0 => 11,
            Self::Key1 => 2,
            Self::Key2 => 3,
            Self::Key3 => 4,
            Self::Key4 => 5,
            Self::Key5 => 6,
            Self::Key6 => 7,
            Self::Key7 => 8,
            Self::Key8 => 9,
            Self::Key9 => 10,

            Self::F1 => 59,
            Self::F2 => 60,
            Self::F3 => 61,
            Self::F4 => 62,
            Self::F5 => 63,
            Self::F6 => 64,
            Self::F7 => 65,
            Self::F8 => 66,
            Self::F9 => 67,
            Self::F10 => 68,
            Self::F11 => 87,
            Self::F12 => 88,
            Self::F13 => 183,
            Self::F14 => 184,
            Self::F15 => 185,
            Self::F16 => 186,
            Self::F17 => 187,
            Self::F18 => 188,
            Self::F19 => 189,
            Self::F20 => 190,
            Self::F21 => 191,
            Self::F22 => 192,
            Self::F23 => 193,
            Self::F24 => 194,

            Self::ControlLeft => 29,
            Self::ControlRight => 97,
            Self::ShiftLeft => 42,
            Self::ShiftRight => 54,
            Self::AltLeft => 56,
            Self::AltRight => 100,
            Self::SuperLeft => 125,
            Self::SuperRight => 126,

            Self::Tab => 15,
            Self::Escape => 1,
            Self::Space => 57,
            Self::Enter => 28,
            Self::Backspace => 14,
            Self::Delete => 111,
            Self::Insert => 110,
            Self::Home => 102,
            Self::End => 107,
            Self::PageUp => 104,
            Self::PageDown => 109,
            Self::Up => 103,
            Self::Down => 108,
            Self::Left => 105,
            Self::Right => 106,
            Self::PrintScreen => 99,
            Self::ScrollLock => 70,
            Self::Pause => 119,
            Self::CapsLock => 58,

            Self::Minus => 12,
            Self::Equal => 13,
            Self::LeftBracket => 26,
            Self::RightBracket => 27,
            Self::Backslash => 43,
            Self::Semicolon => 39,
            Self::Quote => 40,
            Self::Comma => 51,
            Self::Period => 52,
            Self::Slash => 53,
            Self::Backtick => 41,

            Self::Numpad0 => 82,
            Self::Numpad1 => 79,
            Self::Numpad2 => 80,
            Self::Numpad3 => 81,
            Self::Numpad4 => 75,
            Self::Numpad5 => 76,
            Self::Numpad6 => 77,
            Self::Numpad7 => 71,
            Self::Numpad8 => 72,
            Self::Numpad9 => 73,
            Self::NumpadAdd => 78,
            Self::NumpadSubtract => 74,
            Self::NumpadMultiply => 55,
            Self::NumpadDivide => 98,
            Self::NumpadDecimal => 83,
            Self::NumpadEnter => 96,
            Self::NumpadLock => 69,

            Self::MediaPlay => 164,
            Self::MediaPause => 164,
            Self::MediaStop => 166,
            Self::MediaNext => 163,
            Self::MediaPrev => 165,
            Self::VolumeUp => 115,
            Self::VolumeDown => 114,
            Self::Mute => 113,

            Self::BrowserBack => 158,
            Self::BrowserForward => 159,
            Self::BrowserRefresh => 181,

            Self::Meta => 0,
            Self::Unknown(_) => 0,
        }
    }

    /// Returns true if this key is a modifier key.
    pub fn is_modifier(&self) -> bool {
        matches!(
            self,
            Self::ShiftLeft
                | Self::ShiftRight
                | Self::ControlLeft
                | Self::ControlRight
                | Self::AltLeft
                | Self::AltRight
                | Self::SuperLeft
                | Self::SuperRight
                | Self::Meta
        )
    }

    /// Returns true if this key is a letter (A-Z).
    pub fn is_letter(&self) -> bool {
        matches!(
            self,
            Self::A
                | Self::B
                | Self::C
                | Self::D
                | Self::E
                | Self::F
                | Self::G
                | Self::H
                | Self::I
                | Self::J
                | Self::K
                | Self::L
                | Self::M
                | Self::N
                | Self::O
                | Self::P
                | Self::Q
                | Self::R
                | Self::S
                | Self::T
                | Self::U
                | Self::V
                | Self::W
                | Self::X
                | Self::Y
                | Self::Z
        )
    }

    /// Returns true if this is a numpad digit key (Numpad0-Numpad9).
    pub fn is_numpad_digit(&self) -> bool {
        matches!(
            self,
            Self::Numpad0
                | Self::Numpad1
                | Self::Numpad2
                | Self::Numpad3
                | Self::Numpad4
                | Self::Numpad5
                | Self::Numpad6
                | Self::Numpad7
                | Self::Numpad8
                | Self::Numpad9
        )
    }

    /// Returns the shifted character for this key if Shift is held,
    /// otherwise returns None.
    pub fn shifted_label(&self) -> Option<String> {
        match self {
            // Top row numbers
            Self::Key1 => Some("!".into()),
            Self::Key2 => Some("@".into()),
            Self::Key3 => Some("#".into()),
            Self::Key4 => Some("$".into()),
            Self::Key5 => Some("%".into()),
            Self::Key6 => Some("^".into()),
            Self::Key7 => Some("&".into()),
            Self::Key8 => Some("*".into()),
            Self::Key9 => Some("(".into()),
            Self::Key0 => Some(")".into()),
            // Punctuation & symbols
            Self::Minus => Some("_".into()),
            Self::Equal => Some("+".into()),
            Self::LeftBracket => Some("{".into()),
            Self::RightBracket => Some("}".into()),
            Self::Backslash => Some("|".into()),
            Self::Semicolon => Some(":".into()),
            Self::Quote => Some("\"".into()),
            Self::Comma => Some("<".into()),
            Self::Period => Some(">".into()),
            Self::Slash => Some("?".into()),
            Self::Backtick => Some("~".into()),
            _ => None,
        }
    }

    /// Returns the NumLock-off (navigation) label for numpad keys.
    pub fn numlock_off_label(&self) -> Option<String> {
        match self {
            Self::Numpad0 => Some("Ins".into()),
            Self::Numpad1 => Some("End".into()),
            Self::Numpad2 => Some("Down".into()),
            Self::Numpad3 => Some("PgDn".into()),
            Self::Numpad4 => Some("Left".into()),
            Self::Numpad5 => Some("Num5".into()), // No nav equivalent
            Self::Numpad6 => Some("Right".into()),
            Self::Numpad7 => Some("Home".into()),
            Self::Numpad8 => Some("Up".into()),
            Self::Numpad9 => Some("PgUp".into()),
            Self::NumpadDecimal => Some("Del".into()),
            Self::NumpadEnter => Some("Enter".into()),
            Self::NumpadAdd => Some("Num+".into()),
            Self::NumpadSubtract => Some("Num-".into()),
            Self::NumpadMultiply => Some("Num*".into()),
            Self::NumpadDivide => Some("Num/".into()),
            _ => None,
        }
    }
}

impl fmt::Display for VirtualKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}
