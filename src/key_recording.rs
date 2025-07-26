use iced::{keyboard, Event, Subscription};
use iced::keyboard::KeyCode;
use crate::interface::Message;

pub fn key_recorder() -> Subscription<Message> {
    iced::subscription::events_with(|event, _status| {
        match event {
            Event::Keyboard(keyboard::Event::KeyPressed { key_code, modifiers, .. }) => {
                // Ignore modifier-only keys
                if is_modifier_key(&key_code) {
                    return None;
                }

                // Build modifiers list in canonical lowercase form
                let mut combo_parts = Vec::new();
                if modifiers.control() { combo_parts.push("ctrl"); }
                if modifiers.alt() { combo_parts.push("alt"); }
                if modifiers.shift() { combo_parts.push("shift"); }
                if modifiers.logo() { combo_parts.push("super"); }

                // Map KeyCode to canonical key name
                if let Some(key_str) = key_code_to_string(&key_code) {
                    combo_parts.push(key_str);
                } else {
                    // Unrecognized key, ignore
                    return None;
                }

                let combination = combo_parts.join(" + ");
                Some(Message::KeyRecorded(combination))
            }
            _ => None,
        }
    })
}

fn is_modifier_key(key: &KeyCode) -> bool {
    matches!(
        key,
        KeyCode::LControl
            | KeyCode::RControl
            | KeyCode::LAlt
            | KeyCode::RAlt
            | KeyCode::LShift
            | KeyCode::RShift
            | KeyCode::LWin
            | KeyCode::RWin
    )
}

/// Map iced::keyboard::KeyCode enum to lowercase strings as per SWHKD format.
/// Returns None for unsupported keys.
fn key_code_to_string(key_code: &KeyCode) -> Option<&'static str> {
    use KeyCode::*;

    Some(match key_code {
        A => "a",
        B => "b",
        C => "c",
        D => "d",
        E => "e",
        F => "f",
        G => "g",
        H => "h",
        I => "i",
        J => "j",
        K => "k",
        L => "l",
        M => "m",
        N => "n",
        O => "o",
        P => "p",
        Q => "q",
        R => "r",
        S => "s",
        T => "t",
        U => "u",
        V => "v",
        W => "w",
        X => "x",
        Y => "y",
        Z => "z",
        Key1 => "1",
        Key2 => "2",
        Key3 => "3",
        Key4 => "4",
        Key5 => "5",
        Key6 => "6",
        Key7 => "7",
        Key8 => "8",
        Key9 => "9",
        Key0 => "0",
        Escape => "escape",
        Return => "enter",
        Tab => "tab",
        Space => "space",
        Minus => "minus",
        Equal => "equal",
        Backslash => "backslash",
        LBracket => "bracketleft",
        RBracket => "bracketright",
        Semicolon => "semicolon",
        Apostrophe => "apostrophe",
        Comma => "comma",
        Period => "dot",
        Slash => "slash",
        F1 => "f1",
        F2 => "f2",
        F3 => "f3",
        F4 => "f4",
        F5 => "f5",
        F6 => "f6",
        F7 => "f7",
        F8 => "f8",
        F9 => "f9",
        F10 => "f10",
        F11 => "f11",
        F12 => "f12",
        Insert => "insert",
        Delete => "delete",
        Home => "home",
        End => "end",
        PageUp => "page_up",
        PageDown => "page_down",
        Left => "left",
        Right => "right",
        Up => "up",
        Down => "down",
        Backspace => "backspace",
        // Add more keys if you need
        _ => return None,
    })
}
