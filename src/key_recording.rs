use iced::{keyboard, Event, Subscription};
use iced::keyboard::KeyCode;
use crate::interface::Message;

pub fn key_recorder() -> Subscription<Message> {
    iced::subscription::events_with(|event, _status| {
        match event {
            Event::Keyboard(keyboard::Event::KeyPressed { key_code, modifiers, .. }) => {
                if is_modifier_key(&key_code) {
                    return None;
                }
                let mut combo_parts = Vec::new();
                if modifiers.control() { combo_parts.push("Ctrl".to_string()); }
                if modifiers.alt() { combo_parts.push("Alt".to_string()); }
                if modifiers.shift() { combo_parts.push("Shift".to_string()); }
                if modifiers.logo() { combo_parts.push("Super".to_string()); }
                let key_str = format!("{:?}", key_code);
                combo_parts.push(key_str);
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