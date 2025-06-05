use serde::{Serialize, Deserialize};
use std::collections::BTreeSet;

// Translation/i18n support - simple approach
pub fn tr(key: &str) -> String {
    match key {
        "app_name" => "App Name".to_string(),
        "apps" => "APPS".to_string(),
        "key_combination" => "Key Combination".to_string(),
        "command" => "Command".to_string(),
        "active" => "Active".to_string(),
        "delete" => "Delete".to_string(),
        "record" => "Record".to_string(),
        "add_hotkey" => "Add Hotkey".to_string(),
        "add_app" => "+ Add App".to_string(),
        "no_apps_available" => "No apps available. Please add an app.".to_string(),
        "swhkd_gui_configurator" => "SWHKD GUI Configurator".to_string(),
        "recording" => "🔴".to_string(),
        "not_recording" => "⎈".to_string(),
        _ => key.to_string(), // Fallback
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub command: String,
    pub active: bool,
    pub layer_id: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hotkey {
    pub modifiers: BTreeSet<String>,
    pub key: String,
    pub action: Action,
}

impl std::fmt::Display for Hotkey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.modifiers.is_empty() {
            write!(f, "{}", self.key)
        } else {
            write!(
                f,
                "{} + {}",
                self.modifiers.iter().cloned().collect::<Vec<_>>().join(" + "),
                self.key
            )
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppMode {
    pub name: String,
    pub hotkeys: Vec<Hotkey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub apps: Vec<AppMode>,
    pub selected_app: usize,
    pub recording_hotkey: Option<usize>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            apps: vec![
                AppMode {
                    name: "App 1".to_string(),
                    hotkeys: vec![],
                }
            ],
            selected_app: 0,
            recording_hotkey: None,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }
}
