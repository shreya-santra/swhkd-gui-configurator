mod data_model;
mod interface;

use iced::{Application, Command, Element, Settings, Theme};
use data_model::AppState;
use interface::{view, Message};
use sweet_git::{Modifier, Key};
use evdev::KeyCode as EV_KEY;

pub fn main() -> iced::Result {
    SwhkdGui::run(Settings::default())
}

struct SwhkdGui {
    state: AppState,
    error: Option<String>,
}

impl Application for SwhkdGui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let mut state = AppState::new();
        let _ = state.load_from_swhkd_config();
        (Self { state, error: None }, Command::none())
    }

    fn title(&self) -> String {
        "SWHKD GUI Configurator".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::EditKey(idx, new_key) => {
                let mut parts: Vec<&str> = new_key.split('+').map(|s| s.trim()).collect();
                if let Some(last) = parts.pop() {
                    // You may need to map `last` to an appropriate evdev::enums::EV_KEY variant
                    // For demonstration, assuming EV_KEY::KEY_A for 'a'
                    let evdev_keycode = match last.to_lowercase().as_str() {
                        "a" => EV_KEY::KEY_A,
                        "b" => EV_KEY::KEY_B,
                        "c" => EV_KEY::KEY_C,
                        // Add more mappings as needed
                        _ => EV_KEY::KEY_A, // default fallback
                    };
                    let key = Key::new(evdev_keycode, sweet_git::KeyAttribute::empty());
                    let modifiers = parts.iter().filter_map(|m| match *m {
                        "Ctrl" => Some(Modifier::Control),
                        "Alt" => Some(Modifier::Alt),
                        "Shift" => Some(Modifier::Shift),
                        "Super" => Some(Modifier::Super),
                        _ => None,
                    }).collect();
                    if let Some(hotkey) = self.state.working.get_mut(idx) {
                        hotkey.key = key;
                        hotkey.modifiers = modifiers;
                    }
                }
            }
            Message::EditCommand(idx, new_command) => {
                if let Some(hotkey) = self.state.working.get_mut(idx) {
                    hotkey.command = new_command;
                }
            }
            Message::ToggleActive(idx, active) => {
                if let Some(hotkey) = self.state.working.get_mut(idx) {
                    hotkey.active = active;
                }
            }
            Message::DeleteHotkey(idx) => {
                if idx < self.state.working.len() {
                    self.state.working.remove(idx);
                }
            }
            Message::AddHotkey => {
                self.state.working.push(data_model::GuiBinding {
                    modifiers: vec![],
                    key: Key::new(evdev::Key::new(EV_KEY::KEY_A), sweet_git::KeyAttribute::empty()),
                    command: String::from("echo hello"),
                    active: true,
                });
            }
            Message::SaveConfig => {
                match self.state.save_to_swhkd_config() {
                    Ok(_) => self.error = None,
                    Err(e) => self.error = Some(e),
                }
            }
            Message::LoadConfig => {
                let _ = self.state.load_from_swhkd_config();
            }
            Message::ShowError(msg) => self.error = Some(msg),
            Message::ClearError => self.error = None,
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        view(&self.state, &self.error)
    }
}
