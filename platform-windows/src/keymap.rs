use input_core::keys::VirtualKey;

/// Map Windows virtual key code (VK_*) to VirtualKey.
pub fn vk_to_key(vk: u32) -> VirtualKey {
    match vk {
        0x41 => VirtualKey::A,
        0x42 => VirtualKey::B,
        0x43 => VirtualKey::C,
        0x44 => VirtualKey::D,
        0x45 => VirtualKey::E,
        0x46 => VirtualKey::F,
        0x47 => VirtualKey::G,
        0x48 => VirtualKey::H,
        0x49 => VirtualKey::I,
        0x4A => VirtualKey::J,
        0x4B => VirtualKey::K,
        0x4C => VirtualKey::L,
        0x4D => VirtualKey::M,
        0x4E => VirtualKey::N,
        0x4F => VirtualKey::O,
        0x50 => VirtualKey::P,
        0x51 => VirtualKey::Q,
        0x52 => VirtualKey::R,
        0x53 => VirtualKey::S,
        0x54 => VirtualKey::T,
        0x55 => VirtualKey::U,
        0x56 => VirtualKey::V,
        0x57 => VirtualKey::W,
        0x58 => VirtualKey::X,
        0x59 => VirtualKey::Y,
        0x5A => VirtualKey::Z,

        0x30 => VirtualKey::Key0,
        0x31 => VirtualKey::Key1,
        0x32 => VirtualKey::Key2,
        0x33 => VirtualKey::Key3,
        0x34 => VirtualKey::Key4,
        0x35 => VirtualKey::Key5,
        0x36 => VirtualKey::Key6,
        0x37 => VirtualKey::Key7,
        0x38 => VirtualKey::Key8,
        0x39 => VirtualKey::Key9,

        0x70 => VirtualKey::F1,
        0x71 => VirtualKey::F2,
        0x72 => VirtualKey::F3,
        0x73 => VirtualKey::F4,
        0x74 => VirtualKey::F5,
        0x75 => VirtualKey::F6,
        0x76 => VirtualKey::F7,
        0x77 => VirtualKey::F8,
        0x78 => VirtualKey::F9,
        0x79 => VirtualKey::F10,
        0x7A => VirtualKey::F11,
        0x7B => VirtualKey::F12,
        0x7C => VirtualKey::F13,
        0x7D => VirtualKey::F14,
        0x7E => VirtualKey::F15,
        0x7F => VirtualKey::F16,
        0x80 => VirtualKey::F17,
        0x81 => VirtualKey::F18,
        0x82 => VirtualKey::F19,
        0x83 => VirtualKey::F20,
        0x84 => VirtualKey::F21,
        0x85 => VirtualKey::F22,
        0x86 => VirtualKey::F23,
        0x87 => VirtualKey::F24,

        0xA0 => VirtualKey::ShiftLeft,
        0xA1 => VirtualKey::ShiftRight,
        0xA2 => VirtualKey::ControlLeft,
        0xA3 => VirtualKey::ControlRight,
        0xA4 => VirtualKey::AltLeft,
        0xA5 => VirtualKey::AltRight,
        0x5B => VirtualKey::SuperLeft,
        0x5C => VirtualKey::SuperRight,

        0x09 => VirtualKey::Tab,
        0x1B => VirtualKey::Escape,
        0x20 => VirtualKey::Space,
        0x0D => VirtualKey::Enter,
        0x08 => VirtualKey::Backspace,
        0x2E => VirtualKey::Delete,
        0x2D => VirtualKey::Insert,
        0x24 => VirtualKey::Home,
        0x23 => VirtualKey::End,
        0x21 => VirtualKey::PageUp,
        0x22 => VirtualKey::PageDown,
        0x26 => VirtualKey::Up,
        0x28 => VirtualKey::Down,
        0x25 => VirtualKey::Left,
        0x27 => VirtualKey::Right,
        0x2C => VirtualKey::PrintScreen,
        0x91 => VirtualKey::ScrollLock,
        0x13 => VirtualKey::Pause,
        0x14 => VirtualKey::CapsLock,

        0xBD => VirtualKey::Minus,
        0xBB => VirtualKey::Equal,
        0xDB => VirtualKey::LeftBracket,
        0xDD => VirtualKey::RightBracket,
        0xDC => VirtualKey::Backslash,
        0xBA => VirtualKey::Semicolon,
        0xDE => VirtualKey::Quote,
        0xBC => VirtualKey::Comma,
        0xBE => VirtualKey::Period,
        0xBF => VirtualKey::Slash,
        0xC0 => VirtualKey::Backtick,

        0x60 => VirtualKey::Numpad0,
        0x61 => VirtualKey::Numpad1,
        0x62 => VirtualKey::Numpad2,
        0x63 => VirtualKey::Numpad3,
        0x64 => VirtualKey::Numpad4,
        0x65 => VirtualKey::Numpad5,
        0x66 => VirtualKey::Numpad6,
        0x67 => VirtualKey::Numpad7,
        0x68 => VirtualKey::Numpad8,
        0x69 => VirtualKey::Numpad9,
        0x6B => VirtualKey::NumpadAdd,
        0x6D => VirtualKey::NumpadSubtract,
        0x6A => VirtualKey::NumpadMultiply,
        0x6F => VirtualKey::NumpadDivide,
        0x6E => VirtualKey::NumpadDecimal,
        0x90 => VirtualKey::NumpadLock,

        0xB3 => VirtualKey::MediaPlay,
        0xB2 => VirtualKey::MediaStop,
        0xB0 => VirtualKey::MediaNext,
        0xB1 => VirtualKey::MediaPrev,
        0xAD => VirtualKey::Mute,
        0xAE => VirtualKey::VolumeDown,
        0xAF => VirtualKey::VolumeUp,

        0xA6 => VirtualKey::BrowserBack,
        0xA7 => VirtualKey::BrowserForward,
        0xA8 => VirtualKey::BrowserRefresh,

        _ => VirtualKey::Unknown(vk),
    }
}

/// Map Windows virtual key code to VirtualKey (using scan code for disambiguation).
pub fn vk_with_scancode_to_key(vk: u32, scan_code: u32) -> VirtualKey {
    // For left/right modifier disambiguation, use extended key bit
    match vk {
        0xA0 => VirtualKey::ShiftLeft,
        0xA1 => VirtualKey::ShiftRight,
        0xA2 => VirtualKey::ControlLeft,
        0xA3 => VirtualKey::ControlRight,
        0xA4 => VirtualKey::AltLeft,
        0xA5 => VirtualKey::AltRight,
        // For Numpad Enter, check scan code
        0x0D if scan_code == 0xE0 => VirtualKey::NumpadEnter,
        _ => vk_to_key(vk),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_keys() {
        assert_eq!(vk_to_key(0x41), VirtualKey::A);
        assert_eq!(vk_to_key(0x5A), VirtualKey::Z);
        assert_eq!(vk_to_key(0x30), VirtualKey::Key0);
        assert_eq!(vk_to_key(0x39), VirtualKey::Key9);
        assert_eq!(vk_to_key(0x70), VirtualKey::F1);
        assert_eq!(vk_to_key(0x7B), VirtualKey::F12);
        assert_eq!(vk_to_key(0xA2), VirtualKey::ControlLeft);
        assert_eq!(vk_to_key(0x20), VirtualKey::Space);
        assert_eq!(vk_to_key(0x0D), VirtualKey::Enter);
        assert_eq!(vk_to_key(0x1B), VirtualKey::Escape);
    }

    #[test]
    fn test_unknown_vk() {
        assert_eq!(vk_to_key(0xFFFF), VirtualKey::Unknown(0xFFFF));
    }
}
