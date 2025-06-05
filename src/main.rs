mod data_model;
mod interface;
mod key_recording;

use iced::{Application, Command, Element, Settings, Subscription, Theme};
use data_model::{AppState, tr};
use interface::{view, Message};
use std::collections::BTreeSet;
use std::process::Command as ProcessCommand;

pub fn main() -> iced::Result {
    SwhkdGui::run(Settings {
        window: iced::window::Settings {
            size: (1200, 800), // Better default size
            min_size: Some((800, 600)), // Minimum size for responsive design
            resizable: true,
            ..Default::default()
        },
        ..Default::default()
    })
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
        (Self { state: AppState::new() }, Command::none())
    }

    fn title(&self) -> String {
        tr("swhkd_gui_configurator")
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
                    key: String::new(),
                    action: data_model::Action {
                        command: String::new(),
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
                let combination_clone = combination.clone();
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
                // Run the command if the key combo matches an active hotkey (for demo, while GUI focused)
                let app = &self.state.apps[self.state.selected_app];
                for hotkey in &app.hotkeys {
                    if hotkey.action.active
                        && !hotkey.action.command.is_empty()
                        && format!("{}", hotkey) == combination_clone
                    {
                        let _ = ProcessCommand::new("sh")
                            .arg("-c")
                            .arg(&hotkey.action.command)
                            .spawn();
                    }
                }
            }
            Message::StopRecording => {
                self.state.recording_hotkey = None;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        view(&self.state)
    }
}