mod data_model;
mod interface;
use iced::{Application, Command, Element, Settings, Theme};
use data_model::{AppState, AppMode};
use interface::{view, Message};

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
            EditKey(idx, new_key) => {
                let app = &mut self.state.modes[self.state.selected_mode];
                if let Some(hk) = app.hotkeys.get_mut(idx) {
                    hk.key = new_key;
                }
            }
            EditCommand(idx, new_command) => {
                let app = &mut self.state.modes[self.state.selected_mode];
                if let Some(hk) = app.hotkeys.get_mut(idx) {
                    hk.action.command = new_command;
                }
            }
            ToggleActive(idx, active) => {
                let app = &mut self.state.modes[self.state.selected_mode];
                if let Some(hk) = app.hotkeys.get_mut(idx) {
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
                    key: "a".to_string(),
                    action: data_model::GuiAction {
                        command: "".to_string(),
                        active: true,
                        layer_id: 0,
                    }
                });
            }
            AddMode => {
                self.state.modes.push(AppMode {
                    name: "New Mode".to_string(),
                    hotkeys: vec![],
                });
                self.state.selected_mode = self.state.modes.len() - 1;
            }
            StartRecording(idx) => {
                self.state.recording_hotkey = Some(idx);
            }
            KeyRecorded(combo) => {
                // TODO: Implement your key recording logic here
                // This callback is where you parse "a", "ctrl + t", etc.
                // into modifier/key and update the hotkey at `state.recording_hotkey`
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
            LoadConfig => {
                let _ = self.state.load_from_swhkd_config();
            }
            ShowError(msg) => self.error = Some(msg),
            ClearError => self.error = None,
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        view(&self.state, &self.error)
    }
}
