#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum KeyboardInputs {
    Mod(Mod),
    Key(Key),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Mod {
    LeftControl = 0x01,
    LeftShift = 0x02,
    LeftAlt = 0x04,
    LeftMeta = 0x08,
    RightControl = 0x10,
    RightShift = 0x20,
    RightAlt = 0x40,
    RightMeta = 0x80,
}
impl From<Mod> for u8 {
    fn from(value: Mod) -> Self {
        match value {
            Mod::LeftControl => 0x01,
            Mod::LeftShift => 0x02,
            Mod::LeftAlt => 0x04,
            Mod::LeftMeta => 0x08,
            Mod::RightControl => 0x10,
            Mod::RightShift => 0x20,
            Mod::RightAlt => 0x40,
            Mod::RightMeta => 0x80,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Key {
    NONE = 0x00,
    A = 0x04,
    B = 0x05,
    C = 0x06,
    D = 0x07,
    E = 0x08,
    F = 0x09,
    G = 0x0a,
    H = 0x0b,
    I = 0x0c,
    J = 0x0d,
    K = 0x0e,
    L = 0x0f,
    M = 0x10,
    N = 0x11,
    O = 0x12,
    P = 0x13,
    Q = 0x14,
    R = 0x15,
    S = 0x16,
    T = 0x17,
    U = 0x18,
    V = 0x19,
    W = 0x1a,
    X = 0x1b,
    Y = 0x1c,
    Z = 0x1d,
    _1 = 0x1e,
    _2 = 0x1f,
    _3 = 0x20,
    _4 = 0x21,
    _5 = 0x22,
    _6 = 0x23,
    _7 = 0x24,
    _8 = 0x25,
    _9 = 0x26,
    _0 = 0x27,
    Enter = 0x28,
    Escape = 0x29,
    BackSpace = 0x2a,
    Tab = 0x2b,
    Space = 0x2c,
    Minus = 0x2d,
    Equal = 0x2e,
    LeftBrace = 0x2f,
    RightBrace = 0x30,
    BackSlash = 0x31,
    Semicolon = 0x33,
    Quote = 0x34,
    Backtick = 0x35,
    Comma = 0x36,
    Dot = 0x37,
    Slash = 0x38,
    CapsLock = 0x39,
    F1 = 0x3a,
    F2 = 0x3b,
    F3 = 0x3c,
    F4 = 0x3d,
    F5 = 0x3e,
    F6 = 0x3f,
    F7 = 0x40,
    F8 = 0x41,
    F9 = 0x42,
    F10 = 0x43,
    F11 = 0x44,
    F12 = 0x45,
    PrintScreen = 0x46,
    ScrollLock = 0x47,
    Pause = 0x48,
    Insert = 0x49,
    Home = 0x4a,
    PageUp = 0x4b,
    Delete = 0x4c,
    End = 0x4d,
    PageDown = 0x4e,
    Right = 0x4f,
    Left = 0x50,
    Down = 0x51,
    Up = 0x52,
    NumLock = 0x53,
    KeyPadSlash = 0x54,
    KeyPadAsterisk = 0x55,
    KeyPadMinus = 0x56,
    KeyPadPlus = 0x57,
    KeyPadEnter = 0x58,
    KeyPad1 = 0x59,
    KeyPad2 = 0x5a,
    KeyPad3 = 0x5b,
    KeyPad4 = 0x5c,
    KeyPad5 = 0x5d,
    KeyPad6 = 0x5e,
    KeyPad7 = 0x5f,
    KeyPad8 = 0x60,
    KeyPad9 = 0x61,
    KeyPad0 = 0x62,
    KeyPadDelete = 0x63,
    KeyPadCompose = 0x65,
    KeyPadPower = 0x66,
    KeyPadEqual = 0x67,
    F13 = 0x68,
    F14 = 0x69,
    F15 = 0x6a,
    F16 = 0x6b,
    F17 = 0x6c,
    F18 = 0x6d,
    F19 = 0x6e,
    F20 = 0x6f,
    F21 = 0x70,
    F22 = 0x71,
    F23 = 0x72,
    F24 = 0x73,
    Open = 0x74,
    Help = 0x75,
    Props = 0x76,
    Front = 0x77,
    Stop = 0x78,
    Again = 0x79,
    Undo = 0x7a,
    Cut = 0x7b,
    Copy = 0x7c,
    Paste = 0x7d,
    Find = 0x7e,
    Mute = 0x7f,
    VolumeUp = 0x80,
    VolumeDown = 0x81,
    LeftControl = 0xe0,
    LeftShift = 0xe1,
    LeftAlt = 0xe2,
    LeftMeta = 0xe3,
    RightControl = 0xe4,
    RightShift = 0xe5,
    RightAlt = 0xe6,
    RightMeta = 0xe7,
    MediaPlayPause = 0xe8,
    MediaStopCD = 0xe9,
    MediaPrev = 0xea,
    MediaNext = 0xeb,
    MediaEjectCD = 0xec,
    MediaVolumeUp = 0xed,
    MediaVolumeDown = 0xee,
    MediaMute = 0xef,
    MediaWebBrowser = 0xf0,
    MediaBack = 0xf1,
    MediaForward = 0xf2,
    MediaStop = 0xf3,
    MediaFind = 0xf4,
    MediaScrollUp = 0xf5,
    MediaScrollDown = 0xf6,
    MediaEdit = 0xf7,
    MediaSleep = 0xf8,
    MediaCoffee = 0xf9,
    MediaRefresh = 0xfa,
    MediaCalc = 0xfb,
}

impl From<u8> for Key {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Key::NONE,
            0x04 => Key::A,
            0x05 => Key::B,
            0x06 => Key::C,
            0x07 => Key::D,
            0x08 => Key::E,
            0x09 => Key::F,
            0x0a => Key::G,
            0x0b => Key::H,
            0x0c => Key::I,
            0x0d => Key::J,
            0x0e => Key::K,
            0x0f => Key::L,
            0x10 => Key::M,
            0x11 => Key::N,
            0x12 => Key::O,
            0x13 => Key::P,
            0x14 => Key::Q,
            0x15 => Key::R,
            0x16 => Key::S,
            0x17 => Key::T,
            0x18 => Key::U,
            0x19 => Key::V,
            0x1a => Key::W,
            0x1b => Key::X,
            0x1c => Key::Y,
            0x1d => Key::Z,
            0x1e => Key::_1,
            0x1f => Key::_2,
            0x20 => Key::_3,
            0x21 => Key::_4,
            0x22 => Key::_5,
            0x23 => Key::_6,
            0x24 => Key::_7,
            0x25 => Key::_8,
            0x26 => Key::_9,
            0x27 => Key::_0,
            0x28 => Key::Enter,
            0x29 => Key::Escape,
            0x2a => Key::BackSpace,
            0x2b => Key::Tab,
            0x2c => Key::Space,
            0x2d => Key::Minus,
            0x2e => Key::Equal,
            0x2f => Key::LeftBrace,
            0x30 => Key::RightBrace,
            0x31 => Key::BackSlash,
            0x33 => Key::Semicolon,
            0x34 => Key::Quote,
            0x35 => Key::Backtick,
            0x36 => Key::Comma,
            0x37 => Key::Dot,
            0x38 => Key::Slash,
            0x39 => Key::CapsLock,
            0x3a => Key::F1,
            0x3b => Key::F2,
            0x3c => Key::F3,
            0x3d => Key::F4,
            0x3e => Key::F5,
            0x3f => Key::F6,
            0x40 => Key::F7,
            0x41 => Key::F8,
            0x42 => Key::F9,
            0x43 => Key::F10,
            0x44 => Key::F11,
            0x45 => Key::F12,
            0x46 => Key::PrintScreen,
            0x47 => Key::ScrollLock,
            0x48 => Key::Pause,
            0x49 => Key::Insert,
            0x4a => Key::Home,
            0x4b => Key::PageUp,
            0x4c => Key::Delete,
            0x4d => Key::End,
            0x4e => Key::PageDown,
            0x4f => Key::Right,
            0x50 => Key::Left,
            0x51 => Key::Down,
            0x52 => Key::Up,
            0x53 => Key::NumLock,
            0x54 => Key::KeyPadSlash,
            0x55 => Key::KeyPadAsterisk,
            0x56 => Key::KeyPadMinus,
            0x57 => Key::KeyPadPlus,
            0x58 => Key::KeyPadEnter,
            0x59 => Key::KeyPad1,
            0x5a => Key::KeyPad2,
            0x5b => Key::KeyPad3,
            0x5c => Key::KeyPad4,
            0x5d => Key::KeyPad5,
            0x5e => Key::KeyPad6,
            0x5f => Key::KeyPad7,
            0x60 => Key::KeyPad8,
            0x61 => Key::KeyPad9,
            0x62 => Key::KeyPad0,
            0x63 => Key::KeyPadDelete,
            0x65 => Key::KeyPadCompose,
            0x66 => Key::KeyPadPower,
            0x67 => Key::KeyPadEqual,
            0x68 => Key::F13,
            0x69 => Key::F14,
            0x6a => Key::F15,
            0x6b => Key::F16,
            0x6c => Key::F17,
            0x6d => Key::F18,
            0x6e => Key::F19,
            0x6f => Key::F20,
            0x70 => Key::F21,
            0x71 => Key::F22,
            0x72 => Key::F23,
            0x73 => Key::F24,
            0x74 => Key::Open,
            0x75 => Key::Help,
            0x76 => Key::Props,
            0x77 => Key::Front,
            0x78 => Key::Stop,
            0x79 => Key::Again,
            0x7a => Key::Undo,
            0x7b => Key::Cut,
            0x7c => Key::Copy,
            0x7d => Key::Paste,
            0x7e => Key::Find,
            0x7f => Key::Mute,
            0x80 => Key::VolumeUp,
            0x81 => Key::VolumeDown,
            0xe0 => Key::LeftControl,
            0xe1 => Key::LeftShift,
            0xe2 => Key::LeftAlt,
            0xe3 => Key::LeftMeta,
            0xe4 => Key::RightControl,
            0xe5 => Key::RightShift,
            0xe6 => Key::RightAlt,
            0xe7 => Key::RightMeta,
            0xe8 => Key::MediaPlayPause,
            0xe9 => Key::MediaStopCD,
            0xea => Key::MediaPrev,
            0xeb => Key::MediaNext,
            0xec => Key::MediaEjectCD,
            0xed => Key::MediaVolumeUp,
            0xee => Key::MediaVolumeDown,
            0xef => Key::MediaMute,
            0xf0 => Key::MediaWebBrowser,
            0xf1 => Key::MediaBack,
            0xf2 => Key::MediaForward,
            0xf3 => Key::MediaStop,
            0xf4 => Key::MediaFind,
            0xf5 => Key::MediaScrollUp,
            0xf6 => Key::MediaScrollDown,
            0xf7 => Key::MediaEdit,
            0xf8 => Key::MediaSleep,
            0xf9 => Key::MediaCoffee,
            0xfa => Key::MediaRefresh,
            0xfb => Key::MediaCalc,
            _ => Key::NONE,
        }
    }
}

impl From<Key> for u8 {
    fn from(key: Key) -> Self {
        match key {
            Key::NONE => 0x00,
            Key::A => 0x04,
            Key::B => 0x05,
            Key::C => 0x06,
            Key::D => 0x07,
            Key::E => 0x08,
            Key::F => 0x09,
            Key::G => 0x0a,
            Key::H => 0x0b,
            Key::I => 0x0c,
            Key::J => 0x0d,
            Key::K => 0x0e,
            Key::L => 0x0f,
            Key::M => 0x10,
            Key::N => 0x11,
            Key::O => 0x12,
            Key::P => 0x13,
            Key::Q => 0x14,
            Key::R => 0x15,
            Key::S => 0x16,
            Key::T => 0x17,
            Key::U => 0x18,
            Key::V => 0x19,
            Key::W => 0x1a,
            Key::X => 0x1b,
            Key::Y => 0x1c,
            Key::Z => 0x1d,
            Key::_1 => 0x1e,
            Key::_2 => 0x1f,
            Key::_3 => 0x20,
            Key::_4 => 0x21,
            Key::_5 => 0x22,
            Key::_6 => 0x23,
            Key::_7 => 0x24,
            Key::_8 => 0x25,
            Key::_9 => 0x26,
            Key::_0 => 0x27,
            Key::Enter => 0x28,
            Key::Escape => 0x29,
            Key::BackSpace => 0x2a,
            Key::Tab => 0x2b,
            Key::Space => 0x2c,
            Key::Minus => 0x2d,
            Key::Equal => 0x2e,
            Key::LeftBrace => 0x2f,
            Key::RightBrace => 0x30,
            Key::BackSlash => 0x31,
            Key::Semicolon => 0x33,
            Key::Quote => 0x34,
            Key::Backtick => 0x35,
            Key::Comma => 0x36,
            Key::Dot => 0x37,
            Key::Slash => 0x38,
            Key::CapsLock => 0x39,
            Key::F1 => 0x3a,
            Key::F2 => 0x3b,
            Key::F3 => 0x3c,
            Key::F4 => 0x3d,
            Key::F5 => 0x3e,
            Key::F6 => 0x3f,
            Key::F7 => 0x40,
            Key::F8 => 0x41,
            Key::F9 => 0x42,
            Key::F10 => 0x43,
            Key::F11 => 0x44,
            Key::F12 => 0x45,
            Key::PrintScreen => 0x46,
            Key::ScrollLock => 0x47,
            Key::Pause => 0x48,
            Key::Insert => 0x49,
            Key::Home => 0x4a,
            Key::PageUp => 0x4b,
            Key::Delete => 0x4c,
            Key::End => 0x4d,
            Key::PageDown => 0x4e,
            Key::Right => 0x4f,
            Key::Left => 0x50,
            Key::Down => 0x51,
            Key::Up => 0x52,
            Key::NumLock => 0x53,
            Key::KeyPadSlash => 0x54,
            Key::KeyPadAsterisk => 0x55,
            Key::KeyPadMinus => 0x56,
            Key::KeyPadPlus => 0x57,
            Key::KeyPadEnter => 0x58,
            Key::KeyPad1 => 0x59,
            Key::KeyPad2 => 0x5a,
            Key::KeyPad3 => 0x5b,
            Key::KeyPad4 => 0x5c,
            Key::KeyPad5 => 0x5d,
            Key::KeyPad6 => 0x5e,
            Key::KeyPad7 => 0x5f,
            Key::KeyPad8 => 0x60,
            Key::KeyPad9 => 0x61,
            Key::KeyPad0 => 0x62,
            Key::KeyPadDelete => 0x63,
            Key::KeyPadCompose => 0x65,
            Key::KeyPadPower => 0x66,
            Key::KeyPadEqual => 0x67,
            Key::F13 => 0x68,
            Key::F14 => 0x69,
            Key::F15 => 0x6a,
            Key::F16 => 0x6b,
            Key::F17 => 0x6c,
            Key::F18 => 0x6d,
            Key::F19 => 0x6e,
            Key::F20 => 0x6f,
            Key::F21 => 0x70,
            Key::F22 => 0x71,
            Key::F23 => 0x72,
            Key::F24 => 0x73,
            Key::Open => 0x74,
            Key::Help => 0x75,
            Key::Props => 0x76,
            Key::Front => 0x77,
            Key::Stop => 0x78,
            Key::Again => 0x79,
            Key::Undo => 0x7a,
            Key::Cut => 0x7b,
            Key::Copy => 0x7c,
            Key::Paste => 0x7d,
            Key::Find => 0x7e,
            Key::Mute => 0x7f,
            Key::VolumeUp => 0x80,
            Key::VolumeDown => 0x81,
            Key::LeftControl => 0xe0,
            Key::LeftShift => 0xe1,
            Key::LeftAlt => 0xe2,
            Key::LeftMeta => 0xe3,
            Key::RightControl => 0xe4,
            Key::RightShift => 0xe5,
            Key::RightAlt => 0xe6,
            Key::RightMeta => 0xe7,
            Key::MediaPlayPause => 0xe8,
            Key::MediaStopCD => 0xe9,
            Key::MediaPrev => 0xea,
            Key::MediaNext => 0xeb,
            Key::MediaEjectCD => 0xec,
            Key::MediaVolumeUp => 0xed,
            Key::MediaVolumeDown => 0xee,
            Key::MediaMute => 0xef,
            Key::MediaWebBrowser => 0xf0,
            Key::MediaBack => 0xf1,
            Key::MediaForward => 0xf2,
            Key::MediaStop => 0xf3,
            Key::MediaFind => 0xf4,
            Key::MediaScrollUp => 0xf5,
            Key::MediaScrollDown => 0xf6,
            Key::MediaEdit => 0xf7,
            Key::MediaSleep => 0xf8,
            Key::MediaCoffee => 0xf9,
            Key::MediaRefresh => 0xfa,
            Key::MediaCalc => 0xfb,
        }
    }
}

// Converts an ASCII character to the corresponding vector of the KeyboardInputs enum, which is then used to generate the correct HID raw bytes.
pub fn ascii_to_hid(c: char) -> Result<Vec<KeyboardInputs>, String> {
    match c {
        'a'..='z' => Ok(vec![KeyboardInputs::Key(Key::from(c as u8 - b'a' + 0x04))]),
        'A'..='Z' => Ok(vec![
            KeyboardInputs::Key(Key::from(c as u8 - b'A' + 0x04)),
            KeyboardInputs::Key(Key::LeftShift), 
            KeyboardInputs::Mod(Mod::LeftShift),
        ]),
        '1'..='9' => Ok(vec![KeyboardInputs::Key(Key::from(c as u8 - b'1' + 0x1e))]),
        '0' => Ok(vec![KeyboardInputs::Key(Key::_0)]),
        ',' => Ok(vec![KeyboardInputs::Key(Key::Comma)]),
        '?' => Ok(vec![KeyboardInputs::Key(Key::Slash), KeyboardInputs::Key(Key::LeftShift), KeyboardInputs::Mod(Mod::LeftShift),]),
        ' ' => Ok(vec![KeyboardInputs::Key(Key::Space)]),
        '.' => Ok(vec![KeyboardInputs::Key(Key::Dot)]),
        ':' => Ok(vec![KeyboardInputs::Key(Key::Semicolon), KeyboardInputs::Key(Key::LeftShift), KeyboardInputs::Mod(Mod::LeftShift)]),
        '/' => Ok(vec![KeyboardInputs::Key(Key::Slash)]),
        '=' => Ok(vec![KeyboardInputs::Key(Key::Equal)]),
        '"' => Ok(vec![KeyboardInputs::Key(Key::Quote), KeyboardInputs::Key(Key::LeftShift), KeyboardInputs::Mod(Mod::LeftShift)]),
        '\'' => Ok(vec![KeyboardInputs::Key(Key::Quote)]),
        '-' => Ok(vec![KeyboardInputs::Key(Key::Minus)]),
        '+' => Ok(vec![KeyboardInputs::Key(Key::Equal), KeyboardInputs::Key(Key::LeftShift), KeyboardInputs::Mod(Mod::LeftShift)]),
        '\r' | '\n' => Ok(vec![KeyboardInputs::Key(Key::Enter)]),
        _ => Err(format!("Unknown character '{}'", c)),
    }
}

// Generates HID raw bytes from vector of the KeyboardInputs enum.
pub fn hid_raw_bytes(hid_keys: Vec<KeyboardInputs>) -> [u8;11] {
    let mut keycodes: Vec<u8> = Vec::new();
    let mut flags: u8 = 0;
    let mut report:[u8; 11];

    if hid_keys.len() > 7 {
        panic!("Too many keys pressed simultenously !");
    } else if hid_keys.is_empty() {
        report = [0xa1, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    } else {
        for key in hid_keys {
            match key {
                KeyboardInputs::Key(key_value) => {
                    keycodes.push(key_value.into());
                },
                KeyboardInputs::Mod(mod_value) => {
                    let value: u8 = mod_value.into();
                    flags |= value;
                }
            }
        }

        keycodes.extend(vec![0; 7 - keycodes.len()]);

        report= [0xa1, 0x01, flags, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        report[4..].copy_from_slice(&keycodes);

    }
    
    report
}

#[cfg(test)]
mod ascii_to_hid_tests {
    use super::*;

    #[test]
    fn check_simple_min_letter_conv() {
        let a_character_vector = vec![KeyboardInputs::Key(Key::A)];
        let result = ascii_to_hid('a').unwrap();
        assert_eq!(result, a_character_vector);
    }

    #[test]
    fn check_simple_maj_letter_conv() {
        let z_character_vector = vec![KeyboardInputs::Key(Key::Z), KeyboardInputs::Key(Key::LeftShift), KeyboardInputs::Mod(Mod::LeftShift),];
        let result = ascii_to_hid('Z').unwrap();
        assert_eq!(result, z_character_vector);
    }

    #[test]
    fn check_number_conv() {
        let number_two = vec![KeyboardInputs::Key(Key::_2)];
        let result = ascii_to_hid('2').unwrap();
        assert_eq!(result, number_two);
    }

    #[test]
    fn check_zero_conv() {
        let number_two = vec![KeyboardInputs::Key(Key::_0)];
        let result = ascii_to_hid('0').unwrap();
        assert_eq!(result, number_two);
    }

    #[test]
    fn check_cariage_return() {
        let number_two = vec![KeyboardInputs::Key(Key::Enter)];
        let result = ascii_to_hid('\r').unwrap();
        assert_eq!(result, number_two);
    }

    #[test]
    fn check_space_conv() {
        let number_two = vec![KeyboardInputs::Key(Key::Space)];
        let result = ascii_to_hid(' ').unwrap();
        assert_eq!(result, number_two);
    }
}

#[cfg(test)]
mod hid_raw_bytes_tests {
    use super::*;

    #[test]
    fn check_tab_keyboard_report() {
        let tab_hid_report: [u8; 11] = [0xa1, 0x01, 0x00, 0x00, 0x2b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let result: [u8; 11] = hid_raw_bytes(vec![KeyboardInputs::Key(Key::Tab)]);
        assert_eq!(result, tab_hid_report);
    }

    #[test]
    fn check_letter_keyboard_report() {
        let h_letter_report: [u8; 11] = [0xa1, 0x01, 0x00, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let result: [u8; 11] = hid_raw_bytes(vec![KeyboardInputs::Key(Key::H)]);
        assert_eq!(result, h_letter_report);
    }

    #[test]
    fn check_empty_keyboard_report() {
        let empty_keyboard_report: [u8; 11] = [0xa1, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let result = hid_raw_bytes(vec![]);
        assert_eq!(result, empty_keyboard_report);    
    }

    #[test]
    #[should_panic]
    fn check_panic_keyboard_striking() {
        hid_raw_bytes(vec![
            KeyboardInputs::Key(Key::Tab), 
            KeyboardInputs::Key(Key::A), 
            KeyboardInputs::Key(Key::LeftAlt),
            KeyboardInputs::Mod(Mod::LeftAlt),
            KeyboardInputs::Key(Key::B),
            KeyboardInputs::Key(Key::C),
            KeyboardInputs::Key(Key::R),
            KeyboardInputs::Key(Key::E)
            ]);
    }
}