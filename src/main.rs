mod data_model;
mod interface;
mod key_recording;

use iced::{Application, Command, Element, Settings, Subscription, Theme};
use data_model::AppState;
use interface::{view, Message};
use std::collections::BTreeSet;

pub fn main() -> iced::Result {
    SwhkdGui::run(Settings::default())
}

struct SwhkdGui {
    state: AppState,
}

impl Application for SwhkdGui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let mut state = AppState::new();
        let _ = state.load_from_swhkd_config(); // Load existing config if available
        (Self { state }, Command::none())
    }

    fn title(&self) -> String {
        "SWHKD GUI Configurator".to_string()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        if self.state.recording_hotkey.is_some() {
            key_recording::key_recorder()
        } else {
            Subscription::none()
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SelectApp(idx) => self.state.selected_app = idx,
            Message::EditAppName(new_name) => {
                self.state.apps[self.state.selected_app].name = new_name;
            }
            Message::EditKey(idx, new_key) => {
                let app = &mut self.state.apps[self.state.selected_app];
                if let Some(hotkey) = app.hotkeys.get_mut(idx) {
                    if new_key.contains(" + ") {
                        let parts: Vec<&str> = new_key.split(" + ").collect();
                        if let Some((key, modifiers)) = parts.split_last() {
                            hotkey.key = key.to_string();
                            hotkey.modifiers = modifiers.iter().map(|s| s.to_string()).collect();
                        }
                    } else {
                        hotkey.key = new_key;
                        hotkey.modifiers.clear();
                    }
                }
            }
            Message::EditCommand(idx, new_command) => {
                let app = &mut self.state.apps[self.state.selected_app];
                if let Some(hotkey) = app.hotkeys.get_mut(idx) {
                    hotkey.action.command = new_command;
                }
            }
            Message::ToggleActive(idx, active) => {
                let app = &mut self.state.apps[self.state.selected_app];
                if let Some(hotkey) = app.hotkeys.get_mut(idx) {
                    hotkey.action.active = active;
                }
            }
            Message::DeleteHotkey(idx) => {
                let app = &mut self.state.apps[self.state.selected_app];
                if idx < app.hotkeys.len() {
                    app.hotkeys.remove(idx);
                }
            }
            Message::AddHotkey => {
                let app = &mut self.state.apps[self.state.selected_app];
                app.hotkeys.push(data_model::Hotkey {
                    modifiers: BTreeSet::new(),
                    key: String::from(""),
                    action: data_model::Action {
                        command: String::from(""),
                        active: true,
                        layer_id: 0,
                    },
                });
            }
            Message::AddApp => {
                self.state.apps.push(data_model::AppMode {
                    name: format!("App {}", self.state.apps.len() + 1),
                    hotkeys: vec![],
                });
                self.state.selected_app = self.state.apps.len() - 1;
            }
            Message::StartRecording(idx) => {
                self.state.recording_hotkey = Some(idx);
            }
            Message::KeyRecorded(combination) => {
                if let Some(idx) = self.state.recording_hotkey.take() {
                    let app = &mut self.state.apps[self.state.selected_app];
                    if let Some(hotkey) = app.hotkeys.get_mut(idx) {
                        if combination.contains(" + ") {
                            let parts: Vec<&str> = combination.split(" + ").collect();
                            if let Some((key, modifiers)) = parts.split_last() {
                                hotkey.key = key.to_string();
                                hotkey.modifiers = modifiers.iter().map(|s| s.to_string()).collect();
                            }
                        } else {
                            hotkey.key = combination;
                            hotkey.modifiers.clear();
                        }
                    }
                }
            }
            Message::StopRecording => {
                self.state.recording_hotkey = None;
            }
            // NEW: Handle saving to SWHKD
            Message::SaveConfig => {
                match self.state.save_to_swhkd_config() {
                    Ok(()) => println!("✅ Configuration saved and applied to SWHKD!"),
                    Err(e) => println!("❌ Error saving config: {}", e),
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        view(&self.state)
    }
}
