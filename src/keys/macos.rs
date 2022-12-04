// This file is adapted from the original ktrl's `keys.rs` file for macOS.

use rdev::{Event, EventType, Key};
use std::convert::TryFrom;
use std::time::SystemTime;

use super::OsCode;
use super::{KeyEvent, KeyValue};
use crate::keys::macos::keys::*;

impl OsCode {
    pub fn as_u16(self) -> u16 {
        code_from_key(self)
    }

    pub fn from_u16(code: u16) -> Option<Self> {
        code_from_u16(code)
    }

    pub fn from_key(key: Key) -> Option<Self> {
        match key {
            Key::Num0 => Some(OsCode::KEY_0),
            Key::Num1 => Some(OsCode::KEY_1),
            Key::Num2 => Some(OsCode::KEY_2),
            Key::Num3 => Some(OsCode::KEY_3),
            Key::Num4 => Some(OsCode::KEY_4),
            Key::Num5 => Some(OsCode::KEY_5),
            Key::Num6 => Some(OsCode::KEY_6),
            Key::Num7 => Some(OsCode::KEY_7),
            Key::Num8 => Some(OsCode::KEY_8),
            Key::Num9 => Some(OsCode::KEY_9),
            Key::KeyA => Some(OsCode::KEY_A),
            Key::KeyB => Some(OsCode::KEY_B),
            Key::KeyC => Some(OsCode::KEY_C),
            Key::KeyD => Some(OsCode::KEY_D),
            Key::KeyE => Some(OsCode::KEY_E),
            Key::KeyF => Some(OsCode::KEY_F),
            Key::KeyG => Some(OsCode::KEY_G),
            Key::KeyH => Some(OsCode::KEY_H),
            Key::KeyI => Some(OsCode::KEY_I),
            Key::KeyJ => Some(OsCode::KEY_J),
            Key::KeyK => Some(OsCode::KEY_K),
            Key::KeyL => Some(OsCode::KEY_L),
            Key::KeyM => Some(OsCode::KEY_M),
            Key::KeyN => Some(OsCode::KEY_N),
            Key::KeyO => Some(OsCode::KEY_O),
            Key::KeyP => Some(OsCode::KEY_P),
            Key::KeyQ => Some(OsCode::KEY_Q),
            Key::KeyR => Some(OsCode::KEY_R),
            Key::KeyS => Some(OsCode::KEY_S),
            Key::KeyT => Some(OsCode::KEY_T),
            Key::KeyU => Some(OsCode::KEY_U),
            Key::KeyV => Some(OsCode::KEY_V),
            Key::KeyW => Some(OsCode::KEY_W),
            Key::KeyX => Some(OsCode::KEY_X),
            Key::KeyY => Some(OsCode::KEY_Y),
            Key::KeyZ => Some(OsCode::KEY_Z),
            Key::SemiColon => Some(OsCode::KEY_SEMICOLON),
            Key::Slash => Some(OsCode::KEY_SLASH),
            Key::BackQuote => Some(OsCode::KEY_GRAVE),
            Key::LeftBracket => Some(OsCode::KEY_LEFTBRACE),
            Key::BackSlash => Some(OsCode::KEY_BACKSLASH),
            Key::RightBracket => Some(OsCode::KEY_RIGHTBRACE),
            Key::Quote => Some(OsCode::KEY_APOSTROPHE),
            Key::Minus => Some(OsCode::KEY_MINUS),
            Key::Dot => Some(OsCode::KEY_DOT),
            Key::Equal => Some(OsCode::KEY_EQUAL),
            Key::Backspace => Some(OsCode::KEY_BACKSPACE),
            Key::Escape => Some(OsCode::KEY_ESC),
            Key::Tab => Some(OsCode::KEY_TAB),
            Key::Return => Some(OsCode::KEY_ENTER),
            Key::ControlLeft => Some(OsCode::KEY_LEFTCTRL),
            Key::ShiftLeft => Some(OsCode::KEY_LEFTSHIFT),
            Key::Comma => Some(OsCode::KEY_COMMA),
            Key::ShiftRight => Some(OsCode::KEY_RIGHTSHIFT),
            Key::Alt => Some(OsCode::KEY_LEFTALT),
            Key::AltGr => Some(OsCode::KEY_RIGHTALT),
            Key::Space => Some(OsCode::KEY_SPACE),
            Key::CapsLock => Some(OsCode::KEY_CAPSLOCK),
            Key::F1 => Some(OsCode::KEY_F1),
            Key::F2 => Some(OsCode::KEY_F2),
            Key::F3 => Some(OsCode::KEY_F3),
            Key::F4 => Some(OsCode::KEY_F4),
            Key::F5 => Some(OsCode::KEY_F5),
            Key::F6 => Some(OsCode::KEY_F6),
            Key::F7 => Some(OsCode::KEY_F7),
            Key::F8 => Some(OsCode::KEY_F8),
            Key::F9 => Some(OsCode::KEY_F9),
            Key::F10 => Some(OsCode::KEY_F10),
            Key::F11 => Some(OsCode::KEY_F11),
            Key::F12 => Some(OsCode::KEY_F12),
            Key::ScrollLock => Some(OsCode::KEY_SCROLLLOCK),
            Key::KpMinus => Some(OsCode::KEY_KPMINUS),
            Key::KpPlus => Some(OsCode::KEY_KPPLUS),
            Key::UpArrow => Some(OsCode::KEY_UP),
            Key::PageUp => Some(OsCode::KEY_PAGEUP),
            Key::LeftArrow => Some(OsCode::KEY_LEFT),
            Key::RightArrow => Some(OsCode::KEY_RIGHT),
            Key::End => Some(OsCode::KEY_END),
            Key::DownArrow => Some(OsCode::KEY_DOWN),
            Key::PageDown => Some(OsCode::KEY_PAGEDOWN),
            Key::Insert => Some(OsCode::KEY_INSERT),
            Key::Delete => Some(OsCode::KEY_DELETE),
            Key::MetaLeft => Some(OsCode::KEY_LEFTMETA),
            Key::MetaRight => Some(OsCode::KEY_RIGHTMETA),
            Key::ControlRight => Some(OsCode::KEY_RIGHTCTRL),
            Key::Home => Some(OsCode::KEY_HOME),
            Key::PrintScreen => Some(OsCode::KEY_HOME),
            Key::NumLock => Some(OsCode::KEY_NUMLOCK),
            Key::IntlBackslash => Some(OsCode::KEY_HOME),
            Key::KpReturn => Some(OsCode::KEY_ENTER),
            Key::KpMultiply => None,
            Key::KpDivide => None,
            Key::KpDelete => Some(OsCode::KEY_DELETE),
            Key::Function => Some(OsCode::KEY_FN),
            Key::Unknown(10) => Some(OsCode::KEY_102ND),
            Key::Unknown(179) => Some(OsCode::KEY_COMPOSE),
            Key::Unknown(_) => None,
            _ => None
        }
    }

    pub fn as_key(self) -> Key {
        match self {
            OsCode::KEY_0 => Key::Num0,
            OsCode::KEY_1 => Key::Num1,
            OsCode::KEY_2 => Key::Num2,
            OsCode::KEY_3 => Key::Num3,
            OsCode::KEY_4 => Key::Num4,
            OsCode::KEY_5 => Key::Num5,
            OsCode::KEY_6 => Key::Num6,
            OsCode::KEY_7 => Key::Num7,
            OsCode::KEY_8 => Key::Num8,
            OsCode::KEY_9 => Key::Num9,
            OsCode::KEY_A => Key::KeyA,
            OsCode::KEY_B => Key::KeyB,
            OsCode::KEY_C => Key::KeyC,
            OsCode::KEY_D => Key::KeyD,
            OsCode::KEY_E => Key::KeyE,
            OsCode::KEY_F => Key::KeyF,
            OsCode::KEY_G => Key::KeyG,
            OsCode::KEY_H => Key::KeyH,
            OsCode::KEY_I => Key::KeyI,
            OsCode::KEY_J => Key::KeyJ,
            OsCode::KEY_K => Key::KeyK,
            OsCode::KEY_L => Key::KeyL,
            OsCode::KEY_M => Key::KeyM,
            OsCode::KEY_N => Key::KeyN,
            OsCode::KEY_O => Key::KeyO,
            OsCode::KEY_P => Key::KeyP,
            OsCode::KEY_Q => Key::KeyQ,
            OsCode::KEY_R => Key::KeyR,
            OsCode::KEY_S => Key::KeyS,
            OsCode::KEY_T => Key::KeyT,
            OsCode::KEY_U => Key::KeyU,
            OsCode::KEY_V => Key::KeyV,
            OsCode::KEY_W => Key::KeyW,
            OsCode::KEY_X => Key::KeyX,
            OsCode::KEY_Y => Key::KeyY,
            OsCode::KEY_Z => Key::KeyZ,
            OsCode::KEY_SEMICOLON => Key::SemiColon,
            OsCode::KEY_SLASH => Key::Slash,
            OsCode::KEY_GRAVE => Key::BackQuote,
            OsCode::KEY_LEFTBRACE => Key::LeftBracket,
            OsCode::KEY_BACKSLASH => Key::BackSlash,
            OsCode::KEY_RIGHTBRACE => Key::RightBracket,
            OsCode::KEY_APOSTROPHE => Key::Quote,
            OsCode::KEY_MINUS => Key::Minus,
            OsCode::KEY_DOT => Key::Dot,
            OsCode::KEY_EQUAL => Key::Equal,
            OsCode::KEY_BACKSPACE => Key::Backspace,
            OsCode::KEY_ESC => Key::Escape,
            OsCode::KEY_TAB => Key::Tab,
            OsCode::KEY_ENTER => Key::Return,
            OsCode::KEY_LEFTCTRL => Key::ControlLeft,
            OsCode::KEY_LEFTSHIFT => Key::ShiftLeft,
            OsCode::KEY_COMMA => Key::Comma,
            OsCode::KEY_RIGHTSHIFT => Key::ShiftRight,
            OsCode::KEY_LEFTALT => Key::Alt,
            OsCode::KEY_SPACE => Key::Space,
            OsCode::KEY_CAPSLOCK => Key::CapsLock,
            OsCode::KEY_F1 => Key::F1,
            OsCode::KEY_F2 => Key::F2,
            OsCode::KEY_F3 => Key::F3,
            OsCode::KEY_F4 => Key::F4,
            OsCode::KEY_F5 => Key::F5,
            OsCode::KEY_F6 => Key::F6,
            OsCode::KEY_F7 => Key::F7,
            OsCode::KEY_F8 => Key::F8,
            OsCode::KEY_F9 => Key::F9,
            OsCode::KEY_F10 => Key::F10,
            OsCode::KEY_F11 => Key::F11,
            OsCode::KEY_F12 => Key::F12,
            OsCode::KEY_KP0 => Key::Kp0,
            OsCode::KEY_KP1 => Key::Kp1,
            OsCode::KEY_KP2 => Key::Kp2,
            OsCode::KEY_KP3 => Key::Kp3,
            OsCode::KEY_KP4 => Key::Kp4,
            OsCode::KEY_KP5 => Key::Kp5,
            OsCode::KEY_KP6 => Key::Kp6,
            OsCode::KEY_KP7 => Key::Kp7,
            OsCode::KEY_KP8 => Key::Kp8,
            OsCode::KEY_KP9 => Key::Kp9,
            OsCode::KEY_KPMINUS => Key::KpMinus,
            OsCode::KEY_KPPLUS => Key::KpPlus,
            OsCode::KEY_KPDOT => Key::Dot,
            OsCode::KEY_RIGHTCTRL => Key::ControlRight,
            OsCode::KEY_KPSLASH => Key::Slash,
            OsCode::KEY_RIGHTALT => Key::AltGr,
            OsCode::KEY_HOME => Key::Home,
            OsCode::KEY_UP => Key::UpArrow,
            OsCode::KEY_PAGEUP => Key::PageUp,
            OsCode::KEY_LEFT => Key::UpArrow,
            OsCode::KEY_RIGHT => Key::RightArrow,
            OsCode::KEY_END => Key::End,
            OsCode::KEY_DOWN => Key::DownArrow,
            OsCode::KEY_PAGEDOWN => Key::PageDown,
            OsCode::KEY_INSERT => Key::Insert,
            OsCode::KEY_DELETE => Key::Delete,
            OsCode::KEY_LEFTMETA => Key::MetaLeft,
            OsCode::KEY_RIGHTMETA => Key::MetaRight,
            OsCode::KEY_102ND => Key::Unknown(10),
            OsCode::KEY_COMPOSE => Key::Unknown(179),
            _ => Key::Unknown(0),
        }
    }
}

impl TryFrom<Event> for KeyEvent {
    type Error = ();
    fn try_from(item: Event) -> Result<Self, Self::Error> {
        match item.event_type {
            EventType::KeyPress(key) => Ok(Self {
                code: OsCode::from_key(key).ok_or(())?,
                value: KeyValue::Press,
            }),
            EventType::KeyRelease(key) => Ok(Self {
                code: OsCode::from_key(key).ok_or(())?,
                value: KeyValue::Release,
            }),
            _ => Err(()),
        }
    }
}

impl From<KeyEvent> for Event {
    fn from(item: KeyEvent) -> Self {
        Self {
            time: SystemTime::now(),
            name: None,
            event_type: event_type_from_oscode(item.code, item.value),
        }
    }
}

fn event_type_from_oscode(code: OsCode, value: KeyValue) -> EventType {
    match value {
        KeyValue::Release => EventType::KeyRelease(OsCode::as_key(code)),
        KeyValue::Press | KeyValue::Repeat => EventType::KeyPress(OsCode::as_key(code)),
    }
}

#[allow(unused)]
mod keys {
    use core_graphics::event::CGKeyCode;
    use rdev::Key;

    use crate::keys::OsCode;

    // Taken from:
    // https://github.com/Narsil/rdev/blob/main/src/macos/keycodes.rs
    /// Option
    pub const ALT: CGKeyCode = 58;
    /// Option_Right
    pub const ALT_GR: CGKeyCode = 61;
    pub const BACKSPACE: CGKeyCode = 51;
    pub const CAPS_LOCK: CGKeyCode = 57;
    /// Control Right does not exist on Mac
    pub const CONTROL_LEFT: CGKeyCode = 59;
    pub const DOWN_ARROW: CGKeyCode = 125;
    pub const ESCAPE: CGKeyCode = 53;
    pub const F1: CGKeyCode = 122;
    pub const F10: CGKeyCode = 109;
    pub const F11: CGKeyCode = 103;
    pub const F12: CGKeyCode = 111;
    pub const F2: CGKeyCode = 120;
    pub const F3: CGKeyCode = 99;
    pub const F4: CGKeyCode = 118;
    pub const F5: CGKeyCode = 96;
    pub const F6: CGKeyCode = 97;
    pub const F7: CGKeyCode = 98;
    pub const F8: CGKeyCode = 100;
    pub const F9: CGKeyCode = 101;
    pub const FUNCTION: CGKeyCode = 63;
    pub const LEFT_ARROW: CGKeyCode = 123;
    pub const META_LEFT: CGKeyCode = 55;
    pub const META_RIGHT: CGKeyCode = 54;
    pub const RETURN: CGKeyCode = 36;
    pub const RIGHT_ARROW: CGKeyCode = 124;
    pub const SHIFT_LEFT: CGKeyCode = 56;
    pub const SHIFT_RIGHT: CGKeyCode = 60;
    pub const SPACE: CGKeyCode = 49;
    pub const TAB: CGKeyCode = 48;
    pub const UP_ARROW: CGKeyCode = 126;
    pub const BACK_QUOTE: CGKeyCode = 50;
    pub const NUM1: CGKeyCode = 18;
    pub const NUM2: CGKeyCode = 19;
    pub const NUM3: CGKeyCode = 20;
    pub const NUM4: CGKeyCode = 21;
    pub const NUM5: CGKeyCode = 23;
    pub const NUM6: CGKeyCode = 22;
    pub const NUM7: CGKeyCode = 26;
    pub const NUM8: CGKeyCode = 28;
    pub const NUM9: CGKeyCode = 25;
    pub const NUM0: CGKeyCode = 29;
    pub const MINUS: CGKeyCode = 27;
    pub const EQUAL: CGKeyCode = 24;
    pub const KEY_Q: CGKeyCode = 12;
    pub const KEY_W: CGKeyCode = 13;
    pub const KEY_E: CGKeyCode = 14;
    pub const KEY_R: CGKeyCode = 15;
    pub const KEY_T: CGKeyCode = 17;
    pub const KEY_Y: CGKeyCode = 16;
    pub const KEY_U: CGKeyCode = 32;
    pub const KEY_I: CGKeyCode = 34;
    pub const KEY_O: CGKeyCode = 31;
    pub const KEY_P: CGKeyCode = 35;
    pub const LEFT_BRACKET: CGKeyCode = 33;
    pub const RIGHT_BRACKET: CGKeyCode = 30;
    pub const KEY_A: CGKeyCode = 0;
    pub const KEY_S: CGKeyCode = 1;
    pub const KEY_D: CGKeyCode = 2;
    pub const KEY_F: CGKeyCode = 3;
    pub const KEY_G: CGKeyCode = 5;
    pub const KEY_H: CGKeyCode = 4;
    pub const KEY_J: CGKeyCode = 38;
    pub const KEY_K: CGKeyCode = 40;
    pub const KEY_L: CGKeyCode = 37;
    pub const SEMI_COLON: CGKeyCode = 41;
    pub const QUOTE: CGKeyCode = 39;
    pub const BACK_SLASH: CGKeyCode = 42;
    pub const KEY_Z: CGKeyCode = 6;
    pub const KEY_X: CGKeyCode = 7;
    pub const KEY_C: CGKeyCode = 8;
    pub const KEY_V: CGKeyCode = 9;
    pub const KEY_B: CGKeyCode = 11;
    pub const KEY_N: CGKeyCode = 45;
    pub const KEY_M: CGKeyCode = 46;
    pub const COMMA: CGKeyCode = 43;
    pub const DOT: CGKeyCode = 47;
    pub const SLASH: CGKeyCode = 44;
    pub const NON_US_BSLASH: CGKeyCode = 10;
    pub const FN: CGKeyCode = 179;

    pub fn code_from_u16(code: u16) -> Option<OsCode> {
        match code {
            ALT => Some(OsCode::KEY_LEFTALT),
            ALT_GR => Some(OsCode::KEY_RIGHTALT),
            BACKSPACE => Some(OsCode::KEY_BACKSPACE),
            CAPS_LOCK => Some(OsCode::KEY_CAPSLOCK),
            CONTROL_LEFT => Some(OsCode::KEY_LEFTCTRL),
            DOWN_ARROW => Some(OsCode::KEY_DOWN),
            ESCAPE => Some(OsCode::KEY_ESC),
            F1 => Some(OsCode::KEY_F1),
            F10 => Some(OsCode::KEY_F10),
            F11 => Some(OsCode::KEY_F11),
            F12 => Some(OsCode::KEY_F12),
            F2 => Some(OsCode::KEY_F2),
            F3 => Some(OsCode::KEY_F3),
            F4 => Some(OsCode::KEY_F4),
            F5 => Some(OsCode::KEY_F5),
            F6 => Some(OsCode::KEY_F6),
            F7 => Some(OsCode::KEY_F7),
            F8 => Some(OsCode::KEY_F8),
            F9 => Some(OsCode::KEY_F9),
            NUM1 => Some(OsCode::KEY_0),
            NUM2 => Some(OsCode::KEY_2),
            NUM3 => Some(OsCode::KEY_3),
            NUM4 => Some(OsCode::KEY_4),
            NUM5 => Some(OsCode::KEY_5),
            NUM6 => Some(OsCode::KEY_6),
            NUM7 => Some(OsCode::KEY_7),
            NUM8 => Some(OsCode::KEY_8),
            NUM9 => Some(OsCode::KEY_9),
            NUM0 => Some(OsCode::KEY_0),
            LEFT_ARROW => Some(OsCode::BTN_LEFT),
            META_LEFT => Some(OsCode::KEY_LEFTMETA),
            META_RIGHT => Some(OsCode::KEY_RIGHTMETA),
            RETURN => Some(OsCode::KEY_ENTER),
            RIGHT_ARROW => Some(OsCode::BTN_RIGHT),
            SHIFT_LEFT => Some(OsCode::KEY_LEFTSHIFT),
            SHIFT_RIGHT => Some(OsCode::KEY_RIGHTSHIFT),
            SPACE => Some(OsCode::KEY_SPACE),
            TAB => Some(OsCode::KEY_TAB),
            UP_ARROW => Some(OsCode::KEY_UP),
            BACK_QUOTE => Some(OsCode::KEY_GRAVE),
            MINUS => Some(OsCode::KEY_MINUS),
            EQUAL => Some(OsCode::KEY_EQUAL),
            KEY_Q => Some(OsCode::KEY_Q),
            KEY_W => Some(OsCode::KEY_W),
            KEY_E => Some(OsCode::KEY_E),
            KEY_R => Some(OsCode::KEY_R),
            KEY_T => Some(OsCode::KEY_T),
            KEY_Y => Some(OsCode::KEY_Y),
            KEY_U => Some(OsCode::KEY_U),
            KEY_I => Some(OsCode::KEY_I),
            KEY_O => Some(OsCode::KEY_O),
            KEY_P => Some(OsCode::KEY_P),
            LEFT_BRACKET => Some(OsCode::KEY_LEFTBRACE),
            RIGHT_BRACKET => Some(OsCode::KEY_RIGHTBRACE),
            KEY_A => Some(OsCode::KEY_A),
            KEY_S => Some(OsCode::KEY_S),
            KEY_D => Some(OsCode::KEY_D),
            KEY_F => Some(OsCode::KEY_F),
            KEY_G => Some(OsCode::KEY_G),
            KEY_H => Some(OsCode::KEY_H),
            KEY_J => Some(OsCode::KEY_J),
            KEY_K => Some(OsCode::KEY_K),
            KEY_L => Some(OsCode::KEY_L),
            SEMI_COLON => Some(OsCode::KEY_SEMICOLON),
            QUOTE => Some(OsCode::KEY_APOSTROPHE),
            BACK_SLASH => Some(OsCode::KEY_BACKSLASH),
            KEY_Z => Some(OsCode::KEY_Z),
            KEY_X => Some(OsCode::KEY_X),
            KEY_C => Some(OsCode::KEY_C),
            KEY_V => Some(OsCode::KEY_V),
            KEY_B => Some(OsCode::KEY_B),
            KEY_N => Some(OsCode::KEY_N),
            KEY_M => Some(OsCode::KEY_M),
            COMMA => Some(OsCode::KEY_COMMA),
            DOT => Some(OsCode::KEY_DOT),
            SLASH => Some(OsCode::KEY_SLASH),
            FUNCTION => Some(OsCode::KEY_FN),
            NON_US_BSLASH => Some(OsCode::KEY_102ND),
            FN => Some(OsCode::KEY_COMPOSE),
            _code => None,
        }
    }

    pub fn code_from_key(code: OsCode) -> u16 {
        match code {
            OsCode::KEY_LEFTALT => ALT,
            OsCode::KEY_RIGHTALT => ALT_GR,
            OsCode::KEY_BACKSPACE => BACKSPACE,
            OsCode::KEY_CAPSLOCK => CAPS_LOCK,
            OsCode::KEY_LEFTCTRL => CONTROL_LEFT,
            OsCode::KEY_DOWN => DOWN_ARROW,
            OsCode::KEY_ESC => ESCAPE,
            OsCode::KEY_F1 => F1,
            OsCode::KEY_F10 => F10,
            OsCode::KEY_F11 => F11,
            OsCode::KEY_F12 => F12,
            OsCode::KEY_F2 => F2,
            OsCode::KEY_F3 => F3,
            OsCode::KEY_F4 => F4,
            OsCode::KEY_F5 => F5,
            OsCode::KEY_F6 => F6,
            OsCode::KEY_F7 => F7,
            OsCode::KEY_F8 => F8,
            OsCode::KEY_F9 => F9,
            OsCode::KEY_0 => NUM1,
            OsCode::KEY_2 => NUM2,
            OsCode::KEY_3 => NUM3,
            OsCode::KEY_4 => NUM4,
            OsCode::KEY_5 => NUM5,
            OsCode::KEY_6 => NUM6,
            OsCode::KEY_7 => NUM7,
            OsCode::KEY_8 => NUM8,
            OsCode::KEY_9 => NUM9,
            OsCode::KEY_0 => NUM0,
            OsCode::BTN_LEFT => LEFT_ARROW,
            OsCode::KEY_LEFTMETA => META_LEFT,
            OsCode::KEY_RIGHTMETA => META_RIGHT,
            OsCode::KEY_ENTER => RETURN,
            OsCode::BTN_RIGHT => RIGHT_ARROW,
            OsCode::KEY_LEFTSHIFT => SHIFT_LEFT,
            OsCode::KEY_RIGHTSHIFT => SHIFT_RIGHT,
            OsCode::KEY_SPACE => SPACE,
            OsCode::KEY_TAB => TAB,
            OsCode::KEY_UP => UP_ARROW,
            OsCode::KEY_GRAVE => BACK_QUOTE,
            OsCode::KEY_MINUS => MINUS,
            OsCode::KEY_EQUAL => EQUAL,
            OsCode::KEY_Q => KEY_Q,
            OsCode::KEY_W => KEY_W,
            OsCode::KEY_E => KEY_E,
            OsCode::KEY_R => KEY_R,
            OsCode::KEY_T => KEY_T,
            OsCode::KEY_Y => KEY_Y,
            OsCode::KEY_U => KEY_U,
            OsCode::KEY_I => KEY_I,
            OsCode::KEY_O => KEY_O,
            OsCode::KEY_P => KEY_P,
            OsCode::KEY_LEFTBRACE => LEFT_BRACKET,
            OsCode::KEY_RIGHTBRACE => RIGHT_BRACKET,
            OsCode::KEY_A => KEY_A,
            OsCode::KEY_S => KEY_S,
            OsCode::KEY_D => KEY_D,
            OsCode::KEY_F => KEY_F,
            OsCode::KEY_G => KEY_G,
            OsCode::KEY_H => KEY_H,
            OsCode::KEY_J => KEY_J,
            OsCode::KEY_K => KEY_K,
            OsCode::KEY_L => KEY_L,
            OsCode::KEY_SEMICOLON => SEMI_COLON,
            OsCode::KEY_APOSTROPHE => QUOTE,
            OsCode::KEY_BACKSLASH => BACK_SLASH,
            OsCode::KEY_Z => KEY_Z,
            OsCode::KEY_X => KEY_X,
            OsCode::KEY_C => KEY_C,
            OsCode::KEY_V => KEY_V,
            OsCode::KEY_B => KEY_B,
            OsCode::KEY_N => KEY_N,
            OsCode::KEY_M => KEY_M,
            OsCode::KEY_COMMA => COMMA,
            OsCode::KEY_DOT => DOT,
            OsCode::KEY_SLASH => SLASH,
            OsCode::KEY_FN => FUNCTION,
            OsCode::KEY_102ND => NON_US_BSLASH,
            OsCode::KEY_COMPOSE => FN,
            _ => 0,
        }
    }
}
