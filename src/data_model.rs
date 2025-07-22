use sweet::{Binding, Definition, Key, Modifier, SwhkdParser, ParserInput};
use evdev::Key as EvdevKey;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

// Re-export for interface
pub use sweet::Key as SweetKey;

// GUI-friendly structs (your "view model")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiAction {
    pub command: String,
    pub active: bool,
    pub layer_id: usize,
}

// GUI's editable hotkey
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiHotkey {
    pub modifiers: BTreeSet<String>, // e.g. "super", "ctrl"
    pub key: String,                 // e.g. "t", "a"
    pub action: GuiAction,
}

// GUI's editable app/mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppMode {
    pub name: String,
    pub hotkeys: Vec<GuiHotkey>,
}

// GUI's app state (for the window)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub modes: Vec<AppMode>,
    pub selected_mode: usize,
    pub recording_hotkey: Option<usize>,
    pub last_backup: Option<PathBuf>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            modes: vec![AppMode {
                name: "Default".to_string(),
                hotkeys: vec![GuiHotkey {
                    modifiers: [String::from("super")].iter().cloned().collect(),
                    key: String::from("t"),
                    action: GuiAction {
                        command: String::from("alacritty"),
                        active: true,
                        layer_id: 0,
                    },
                }],
            }],
            selected_mode: 0,
            recording_hotkey: None,
            last_backup: None,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    // Load config from disk, parse with sweet, and convert to GUI model
    fn get_swhkd_config_path(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let home = dirs::home_dir().ok_or("Could not find home directory")?;
        Ok(home.join(".config/swhkd/swhkdrc"))
    }

    pub fn load_from_swhkd_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = self.get_swhkd_config_path()?;
        self.modes.clear();

        if config_path.exists() {
            let parser = SwhkdParser::from(ParserInput::Path(&config_path))?;
            // For simplicity, collect all bindings into one "Default" app mode
            let mut hotkeys = Vec::new();
            for binding in parser.bindings {
                let modifiers = binding.definition.modifiers
                    .iter()
                    .map(modifier_to_string)
                    .collect();
                let key = key_to_string(&binding.definition.key);
                hotkeys.push(GuiHotkey {
                    modifiers,
                    key,
                    action: GuiAction {
                        command: binding.command,
                        active: true,
                        layer_id: 0,
                    },
                });
            }
            self.modes.push(AppMode {
                name: "Default".to_string(),
                hotkeys,
            });
            println!("Loaded config from: {}", &config_path.display());
        }

        if self.modes.is_empty() {
            *self = Self::default();
        }
        self.selected_mode = 0;
        Ok(())
    }

    // Save GUI model to disk, convert to sweet::Binding, write as sweet format
    pub fn save_to_swhkd_config(&mut self) -> Result<(), String> {
        // Check for duplicates
        let mut seen = HashSet::new();
        for mode in &self.modes {
            for hk in &mode.hotkeys {
                if hk.action.active {
                    let sig = (hk.modifiers.clone(), hk.key.clone());
                    if !seen.insert(sig) {
                        return Err(format!("Duplicate binding: {} + {}",
                            hk.modifiers.iter().cloned().collect::<Vec<_>>().join(" + "), hk.key));
                    }
                }
            }
        }

        let config_path = self.get_swhkd_config_path().map_err(|e| e.to_string())?;
        // Backup
        if let Some(parent) = config_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if config_path.exists() {
            let backup_path = config_path.with_extension("bak");
            fs::copy(&config_path, &backup_path).map_err(|e| e.to_string())?;
            self.last_backup = Some(backup_path);
        }

        // Convert to sweet::Binding
        let mut bindings = Vec::new();
        for mode in &self.modes {
            for hk in &mode.hotkeys {
                if hk.action.active {
                    let mods = hk.modifiers.iter()
                        .filter_map(|s| string_to_modifier(s.as_str()))
                        .collect::<BTreeSet<_>>();
                    let key = string_to_evdev_key(&hk.key)
                        .map_err(|e| format!("Invalid key in config: {}", e))?;
                    bindings.push(Binding {
                        definition: Definition { modifiers: mods, key: Key { key, attribute: sweet::KeyAttribute::None } },
                        command: hk.action.command.clone(),
                        mode_instructions: Vec::new(),
                    });
                }
            }
        }

        // Write as sweet-compatible config
        let config_text = bindings.iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(&config_path, config_text).map_err(|e| e.to_string())?;

        // Optionally, reload the daemon
        let _ = self.reload_swhkd();
        Ok(())
    }

    fn reload_swhkd(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = Command::new("pkill").arg("-USR1").arg("swhkd").output();
        let _ = Command::new("systemctl").args(["--user", "restart", "swhkd"]).output();
        Ok(())
    }
}

// --- Helper: Conversion between sweet/enums and GUI strings ---

pub fn key_to_string(key: &Key) -> String {
    match key.key {
        EvdevKey::KEY_A => "a".to_string(),
        EvdevKey::KEY_B => "b".to_string(),
        EvdevKey::KEY_C => "c".to_string(),
        EvdevKey::KEY_D => "d".to_string(),
        EvdevKey::KEY_E => "e".to_string(),
        EvdevKey::KEY_F => "f".to_string(),
        EvdevKey::KEY_G => "g".to_string(),
        EvdevKey::KEY_H => "h".to_string(),
        EvdevKey::KEY_I => "i".to_string(),
        EvdevKey::KEY_J => "j".to_string(),
        EvdevKey::KEY_K => "k".to_string(),
        EvdevKey::KEY_L => "l".to_string(),
        EvdevKey::KEY_M => "m".to_string(),
        EvdevKey::KEY_N => "n".to_string(),
        EvdevKey::KEY_O => "o".to_string(),
        EvdevKey::KEY_P => "p".to_string(),
        EvdevKey::KEY_Q => "q".to_string(),
        EvdevKey::KEY_R => "r".to_string(),
        EvdevKey::KEY_S => "s".to_string(),
        EvdevKey::KEY_T => "t".to_string(),
        EvdevKey::KEY_U => "u".to_string(),
        EvdevKey::KEY_V => "v".to_string(),
        EvdevKey::KEY_W => "w".to_string(),
        EvdevKey::KEY_X => "x".to_string(),
        EvdevKey::KEY_Y => "y".to_string(),
        EvdevKey::KEY_Z => "z".to_string(),
        EvdevKey::KEY_1 => "1".to_string(),
        EvdevKey::KEY_2 => "2".to_string(),
        EvdevKey::KEY_3 => "3".to_string(),
        EvdevKey::KEY_4 => "4".to_string(),
        EvdevKey::KEY_5 => "5".to_string(),
        EvdevKey::KEY_6 => "6".to_string(),
        EvdevKey::KEY_7 => "7".to_string(),
        EvdevKey::KEY_8 => "8".to_string(),
        EvdevKey::KEY_9 => "9".to_string(),
        EvdevKey::KEY_0 => "0".to_string(),
        EvdevKey::KEY_ESC => "escape".to_string(),
        EvdevKey::KEY_ENTER => "enter".to_string(),
        EvdevKey::KEY_TAB => "tab".to_string(),
        EvdevKey::KEY_SPACE => "space".to_string(),
        EvdevKey::KEY_MINUS => "minus".to_string(),
        EvdevKey::KEY_EQUAL => "equal".to_string(),
        EvdevKey::KEY_BACKSLASH => "backslash".to_string(),
        EvdevKey::KEY_LEFTBRACE => "bracketleft".to_string(),
        EvdevKey::KEY_RIGHTBRACE => "bracketright".to_string(),
        EvdevKey::KEY_SEMICOLON => "semicolon".to_string(),
        EvdevKey::KEY_APOSTROPHE => "apostrophe".to_string(),
        EvdevKey::KEY_COMMA => "comma".to_string(),
        EvdevKey::KEY_DOT => "dot".to_string(),
        EvdevKey::KEY_SLASH => "slash".to_string(),
        // F1-F12, media, etc. can be added here if needed
        _ => format!("{:?}", key.key),
    }
}

pub fn string_to_evdev_key(s: &str) -> Result<EvdevKey, String> {
    match s {
        "a" | "A" => Ok(EvdevKey::KEY_A),
        "b" | "B" => Ok(EvdevKey::KEY_B),
        "c" | "C" => Ok(EvdevKey::KEY_C),
        "d" | "D" => Ok(EvdevKey::KEY_D),
        "e" | "E" => Ok(EvdevKey::KEY_E),
        "f" | "F" => Ok(EvdevKey::KEY_F),
        "g" | "G" => Ok(EvdevKey::KEY_G),
        "h" | "H" => Ok(EvdevKey::KEY_H),
        "i" | "I" => Ok(EvdevKey::KEY_I),
        "j" | "J" => Ok(EvdevKey::KEY_J),
        "k" | "K" => Ok(EvdevKey::KEY_K),
        "l" | "L" => Ok(EvdevKey::KEY_L),
        "m" | "M" => Ok(EvdevKey::KEY_M),
        "n" | "N" => Ok(EvdevKey::KEY_N),
        "o" | "O" => Ok(EvdevKey::KEY_O),
        "p" | "P" => Ok(EvdevKey::KEY_P),
        "q" | "Q" => Ok(EvdevKey::KEY_Q),
        "r" | "R" => Ok(EvdevKey::KEY_R),
        "s" | "S" => Ok(EvdevKey::KEY_S),
        "t" | "T" => Ok(EvdevKey::KEY_T),
        "u" | "U" => Ok(EvdevKey::KEY_U),
        "v" | "V" => Ok(EvdevKey::KEY_V),
        "w" | "W" => Ok(EvdevKey::KEY_W),
        "x" | "X" => Ok(EvdevKey::KEY_X),
        "y" | "Y" => Ok(EvdevKey::KEY_Y),
        "z" | "Z" => Ok(EvdevKey::KEY_Z),
        _ => Err(format!("Unsupported key: {}", s)),
    }
}

fn modifier_to_string(m: &Modifier) -> String {
    match m {
        Modifier::Super => "super".to_string(),
        Modifier::Alt => "alt".to_string(),
        Modifier::Control => "ctrl".to_string(),
        Modifier::Shift => "shift".to_string(),
        _ => format!("{:?}", m).to_lowercase(),
    }
}

fn string_to_modifier(s: &str) -> Option<Modifier> {
    match s.to_lowercase().as_str() {
        "super" => Some(Modifier::Super),
        "alt" => Some(Modifier::Alt),
        "ctrl" => Some(Modifier::Control),
        "control" => Some(Modifier::Control),
        "shift" => Some(Modifier::Shift),
        _ => None,
    }
}
