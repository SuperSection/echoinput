use input_core::keys::VirtualKey;

/// Map macOS keycode to VirtualKey.
///
/// These are the virtual keycodes used by Carbon/HIToolbox.
/// Reference: Events.h from Carbon framework.
pub fn keycode_to_key(keycode: u32) -> VirtualKey {
    match keycode {
        // Letters (kVK_* values)
        0x00 => VirtualKey::A,
        0x0B => VirtualKey::C,
        0x08 => VirtualKey::D,
        0x0E => VirtualKey::E,
        0x03 => VirtualKey::F,
        0x05 => VirtualKey::G,
        0x04 => VirtualKey::H,
        0x22 => VirtualKey::I,
        0x26 => VirtualKey::J,
        0x28 => VirtualKey::K,
        0x25 => VirtualKey::L,
        0x2E => VirtualKey::M,
        0x2D => VirtualKey::N,
        0x1F => VirtualKey::O,
        0x23 => VirtualKey::P,
        0x0C => VirtualKey::Q,
        0x0F => VirtualKey::R,
        0x01 => VirtualKey::S,
        0x11 => VirtualKey::T,
        0x20 => VirtualKey::U,
        0x09 => VirtualKey::V,
        0x0D => VirtualKey::W,
        0x07 => VirtualKey::X,
        0x10 => VirtualKey::Y,
        0x06 => VirtualKey::Z,

        // Numbers
        0x1D => VirtualKey::Key0,
        0x12 => VirtualKey::Key1,
        0x13 => VirtualKey::Key2,
        0x14 => VirtualKey::Key3,
        0x15 => VirtualKey::Key4,
        0x17 => VirtualKey::Key5,
        0x16 => VirtualKey::Key6,
        0x1A => VirtualKey::Key7,
        0x1C => VirtualKey::Key8,
        0x19 => VirtualKey::Key9,

        // Function keys
        0x7A => VirtualKey::F1,
        0x78 => VirtualKey::F2,
        0x63 => VirtualKey::F3,
        0x76 => VirtualKey::F4,
        0x60 => VirtualKey::F5,
        0x61 => VirtualKey::F6,
        0x62 => VirtualKey::F7,
        0x64 => VirtualKey::F8,
        0x65 => VirtualKey::F9,
        0x6D => VirtualKey::F10,
        0x67 => VirtualKey::F11,
        0x6F => VirtualKey::F12,
        0x69 => VirtualKey::F13,
        0x6B => VirtualKey::F14,
        0x71 => VirtualKey::F15,
        0x6A => VirtualKey::F16,
        0x40 => VirtualKey::F17,
        0x4F => VirtualKey::F18,
        0x50 => VirtualKey::F19,
        0x5A => VirtualKey::F20,

        // Modifiers
        // Note: Left/right determined by key event's isARepeat or we check flags
        0x38 => VirtualKey::ShiftLeft,    // kVK_Shift
        0x3C => VirtualKey::ShiftRight,   // kVK_RightShift
        0x3B => VirtualKey::ControlLeft,  // kVK_Control
        0x3E => VirtualKey::ControlRight, // kVK_RightControl
        0x3A => VirtualKey::AltLeft,      // kVK_Option
        0x3D => VirtualKey::AltRight,     // kVK_RightOption
        0x37 => VirtualKey::SuperLeft,    // kVK_Command
        0x36 => VirtualKey::SuperRight,   // kVK_RightCommand

        // Navigation & editing
        0x30 => VirtualKey::Tab,       // kVK_Tab
        0x35 => VirtualKey::Escape,    // kVK_Escape
        0x31 => VirtualKey::Space,     // kVK_Space
        0x24 => VirtualKey::Enter,     // kVK_Return
        0x33 => VirtualKey::Backspace, // kVK_Delete (Backspace)
        0x75 => VirtualKey::Delete,    // kVK_ForwardDelete
        0x72 => VirtualKey::Insert,    // kVK_Help
        0x73 => VirtualKey::Home,      // kVK_Home
        0x77 => VirtualKey::End,       // kVK_End
        0x74 => VirtualKey::PageUp,    // kVK_PageUp
        0x79 => VirtualKey::PageDown,  // kVK_PageDown
        0x7E => VirtualKey::Up,        // kVK_UpArrow
        0x7D => VirtualKey::Down,      // kVK_DownArrow
        0x7B => VirtualKey::Left,      // kVK_LeftArrow
        0x7C => VirtualKey::Right,     // kVK_RightArrow
        0x47 => VirtualKey::CapsLock,  // kVK_CapsLock

        // Punctuation & symbols
        0x1B => VirtualKey::Minus,        // kVK_Minus
        0x18 => VirtualKey::Equal,        // kVK_Equal
        0x21 => VirtualKey::LeftBracket,  // kVK_OpenBracket
        0x1E => VirtualKey::RightBracket, // kVK_CloseBracket
        0x2A => VirtualKey::Backslash,    // kVK_Backslash
        0x29 => VirtualKey::Semicolon,    // kVK_Semicolon
        0x27 => VirtualKey::Quote,        // kVK_Quote
        0x2B => VirtualKey::Comma,        // kVK_Comma
        0x2F => VirtualKey::Period,       // kVK_Period
        0x2C => VirtualKey::Slash,        // kVK_Slash
        0x32 => VirtualKey::Backtick,     // kVK_Grave

        // Numpad
        0x52 => VirtualKey::Numpad0,
        0x53 => VirtualKey::Numpad1,
        0x54 => VirtualKey::Numpad2,
        0x55 => VirtualKey::Numpad3,
        0x56 => VirtualKey::Numpad4,
        0x57 => VirtualKey::Numpad5,
        0x58 => VirtualKey::Numpad6,
        0x59 => VirtualKey::Numpad7,
        0x5B => VirtualKey::Numpad8,
        0x5C => VirtualKey::Numpad9,
        0x45 => VirtualKey::NumpadAdd,
        0x4E => VirtualKey::NumpadSubtract,
        0x43 => VirtualKey::NumpadMultiply,
        0x4B => VirtualKey::NumpadDivide,
        0x41 => VirtualKey::NumpadDecimal,
        0x4C => VirtualKey::NumpadEnter,

        // Media
        // macOS uses consumer control HID events, not keycodes for media keys
        // But these are the NX_KEYTYPE mapped values
        0x46 => VirtualKey::Mute,       // kVK_Mute (NX_KEYTYPE_MUTE)
        0x48 => VirtualKey::VolumeDown, // kVK_VolumeDown
        0x49 => VirtualKey::VolumeUp,   // kVK_VolumeUp

        _ => VirtualKey::Unknown(keycode),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_keys() {
        assert_eq!(keycode_to_key(0x00), VirtualKey::A);
        assert_eq!(keycode_to_key(0x06), VirtualKey::Z);
        assert_eq!(keycode_to_key(0x1D), VirtualKey::Key0);
        assert_eq!(keycode_to_key(0x19), VirtualKey::Key9);
        assert_eq!(keycode_to_key(0x7A), VirtualKey::F1);
        assert_eq!(keycode_to_key(0x6F), VirtualKey::F12);
        assert_eq!(keycode_to_key(0x3B), VirtualKey::ControlLeft);
        assert_eq!(keycode_to_key(0x31), VirtualKey::Space);
        assert_eq!(keycode_to_key(0x24), VirtualKey::Enter);
        assert_eq!(keycode_to_key(0x35), VirtualKey::Escape);
    }

    #[test]
    fn test_unknown_keycode() {
        assert_eq!(keycode_to_key(0xFFFF), VirtualKey::Unknown(0xFFFF));
    }
}
