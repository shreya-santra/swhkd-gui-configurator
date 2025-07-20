use iced::{keyboard, Subscription};
use crate::interface::Message;
use sweet::ast::{Key, Modifier};

pub fn key_recorder() -> Subscription<Message> {
    keyboard::on_key_press(handle_keypress)
}

fn handle_keypress(key: keyboard::Key, modifiers: keyboard::Modifiers) -> Option<Message> {

    if is_modifier_key(&key) {
        return None;
    }
    
    let mut combo_parts = Vec::new();
    
    
    if modifiers.control { combo_parts.push("Ctrl".to_string()); }
    if modifiers.alt { combo_parts.push("Alt".to_string()); }
    if modifiers.shift { combo_parts.push("Shift".to_string()); }
    if modifiers.logo { combo_parts.push("Super".to_string()); }
    
    
    let key_str = match key.as_ref() {
        keyboard::Key::Character(c) => c.to_string(),
        keyboard::Key::Named(named) => format!("{:?}", named),
        _ => return None, // Skip other key types
    };
    combo_parts.push(key_str);
    
    let combination = combo_parts.join(" + ");
    Some(Message::EditKey(0, combination)) 
}

fn is_modifier_key(key: &keyboard::Key) -> bool {
    matches!(
        key.as_ref(),
        keyboard::Key::Named(keyboard::key::Named::Control)
            | keyboard::Key::Named(keyboard::key::Named::Alt)
            | keyboard::Key::Named(keyboard::key::Named::Shift)
            | keyboard::Key::Named(keyboard::key::Named::Super)
    )
}
