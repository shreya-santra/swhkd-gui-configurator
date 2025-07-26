mod data_model;
mod interface;

use iced::{Application, Command, Element, Settings, Theme, Subscription};
use data_model::{AppState};
use interface::{view, Message};
use iced::keyboard::{self, Event, KeyCode};

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
        use Message::*;
        match message {
            SelectMode(idx) => {
                if idx < self.state.modes.len() {
                    self.state.selected_mode = idx;
                }
            }
            EditModeName(new_name) => {
                if let Some(mode) = self.state.modes.get_mut(self.state.selected_mode) {
                    mode.name = new_name;
                }
            }
            EditCommand(idx, new_command) => {
                if let Some(hk) = self.state.modes[self.state.selected_mode].hotkeys.get_mut(idx) {
                    hk.action.command = new_command;
                }
            }
            ToggleActive(idx, active) => {
                if let Some(hk) = self.state.modes[self.state.selected_mode].hotkeys.get_mut(idx) {
                    hk.action.active = active;
                }
            }
            DeleteHotkey(idx) => {
                let app = &mut self.state.modes[self.state.selected_mode];
                if idx < app.hotkeys.len() {
                    app.hotkeys.remove(idx);
                }
            }
            AddHotkey => {
                let app = &mut self.state.modes[self.state.selected_mode];
                app.hotkeys.push(data_model::GuiHotkey {
                    modifiers: Default::default(),
                    key: String::new(),
                    action: data_model::GuiAction {
                        command: String::new(),
                        active: true,
                        layer_id: 0,
                    },
                });
                self.state.recording_hotkey = Some(app.hotkeys.len() - 1);
            }
            AddMode => {
                self.state.modes.push(data_model::AppMode {
                    name: "New Mode".to_string(),
                    hotkeys: vec![],
                });
                self.state.selected_mode = self.state.modes.len() - 1;
            }
            StartRecording(idx) => {
                self.state.recording_hotkey = Some(idx);
            }
            KeyRecorded(combo) => {
                if let Some(idx) = self.state.recording_hotkey {
                    if let Some(hotkey) = self.state.modes[self.state.selected_mode].hotkeys.get_mut(idx) {
                        let parts: Vec<_> = combo.split('+').map(|s| s.trim().to_string()).collect();
                        if !parts.is_empty() {
                            if parts.len() == 1 {
                                hotkey.modifiers.clear();
                                hotkey.key = parts[0].clone();
                            } else {
                                hotkey.modifiers = parts[..parts.len()-1].iter().cloned().collect();
                                hotkey.key = parts[parts.len()-1].clone();
                            }
                        }
                    }
                }
                self.state.recording_hotkey = None;
            }
            StopRecording => {
                self.state.recording_hotkey = None;
            }
            SaveConfig => {
                match self.state.save_to_swhkd_config() {
                    Ok(_) => self.error = None,
                    Err(e) => self.error = Some(e),
                }
            }
            ShowError(msg) => self.error = Some(msg),
            ClearError => self.error = None,
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        if self.state.recording_hotkey.is_some() {
            iced::subscription::events_with(|event, _status| {
                if let iced::Event::Keyboard(Event::KeyPressed { key_code, modifiers, .. }) = event {
                    let mut parts = Vec::new();
                    if modifiers.control() { parts.push("ctrl".to_string()); }
                    if modifiers.alt() { parts.push("alt".to_string()); }
                    if modifiers.shift() { parts.push("shift".to_string()); }
                    if modifiers.logo() { parts.push("super".to_string()); }

                    // Only accept normal keys, not modifier keys
                    match key_code {
                        KeyCode::A => parts.push("a".to_string()),
                        KeyCode::B => parts.push("b".to_string()),
                        KeyCode::C => parts.push("c".to_string()),
                        KeyCode::D => parts.push("d".to_string()),
                        KeyCode::E => parts.push("e".to_string()),
                        KeyCode::F => parts.push("f".to_string()),
                        KeyCode::G => parts.push("g".to_string()),
                        KeyCode::H => parts.push("h".to_string()),
                        KeyCode::I => parts.push("i".to_string()),
                        KeyCode::J => parts.push("j".to_string()),
                        KeyCode::K => parts.push("k".to_string()),
                        KeyCode::L => parts.push("l".to_string()),
                        KeyCode::M => parts.push("m".to_string()),
                        KeyCode::N => parts.push("n".to_string()),
                        KeyCode::O => parts.push("o".to_string()),
                        KeyCode::P => parts.push("p".to_string()),
                        KeyCode::Q => parts.push("q".to_string()),
                        KeyCode::R => parts.push("r".to_string()),
                        KeyCode::S => parts.push("s".to_string()),
                        KeyCode::T => parts.push("t".to_string()),
                        KeyCode::U => parts.push("u".to_string()),
                        KeyCode::V => parts.push("v".to_string()),
                        KeyCode::W => parts.push("w".to_string()),
                        KeyCode::X => parts.push("x".to_string()),
                        KeyCode::Y => parts.push("y".to_string()),
                        KeyCode::Z => parts.push("z".to_string()),
                        KeyCode::Key1 => parts.push("1".to_string()),
                        KeyCode::Key2 => parts.push("2".to_string()),
                        KeyCode::Key3 => parts.push("3".to_string()),
                        KeyCode::Key4 => parts.push("4".to_string()),
                        KeyCode::Key5 => parts.push("5".to_string()),
                        KeyCode::Key6 => parts.push("6".to_string()),
                        KeyCode::Key7 => parts.push("7".to_string()),
                        KeyCode::Key8 => parts.push("8".to_string()),
                        KeyCode::Key9 => parts.push("9".to_string()),
                        KeyCode::Key0 => parts.push("0".to_string()),
                        KeyCode::Escape => parts.push("escape".to_string()),
                        KeyCode::Enter => parts.push("enter".to_string()),
                        KeyCode::Tab => parts.push("tab".to_string()),
                        KeyCode::Space => parts.push("space".to_string()),
                        KeyCode::Minus => parts.push("minus".to_string()),
                        KeyCode::Equals => parts.push("equal".to_string()),
                        KeyCode::Backslash => parts.push("backslash".to_string()),
                        KeyCode::LBracket => parts.push("bracketleft".to_string()),
                        KeyCode::RBracket => parts.push("bracketright".to_string()),
                        KeyCode::Semicolon => parts.push("semicolon".to_string()),
                        KeyCode::Apostrophe => parts.push("apostrophe".to_string()),
                        KeyCode::Comma => parts.push("comma".to_string()),
                        KeyCode::Period => parts.push("dot".to_string()),
                        KeyCode::Slash => parts.push("slash".to_string()),
                        // ...add function keys as desired...
                        _ => return Some(Message::ClearError),
                    };

                    if parts.is_empty() {
                        Some(Message::ClearError)
                    } else {
                        Some(Message::KeyRecorded(parts.join(" + ")))
                    }
                } else {
                    None
                }
            })
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> Element<Self::Message> {
        view(&self.state, &self.error)
    }
}



