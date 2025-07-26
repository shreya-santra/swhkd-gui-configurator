use sweet::{Binding, Definition, Key, KeyAttribute, Modifier, SwhkdParser, ParserInput};
use evdev::Key as EvdevKey;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiAction {
    pub command: String,
    pub active: bool,
    pub layer_id: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiHotkey {
    pub modifiers: BTreeSet<String>,
    pub key: String,
    pub action: GuiAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppMode {
    pub name: String,
    pub hotkeys: Vec<GuiHotkey>,
}

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
                hotkeys: Vec::new(),
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

    fn get_swhkd_config_path(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let home = dirs::home_dir().ok_or("Could not find home directory")?;
        Ok(home.join(".config/swhkd/swhkdrc"))
    }

    pub fn load_from_swhkd_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = self.get_swhkd_config_path()?;
        self.modes.clear();

        let mut hotkeys = Vec::new();
        if config_path.exists() {
            if let Ok(parser) = SwhkdParser::from(ParserInput::Path(&config_path)) {
                for binding in parser.bindings {
                    hotkeys.push(GuiHotkey {
                        modifiers: binding.definition.modifiers.iter().map(modifier_to_string).collect(),
                        key: key_to_string(&binding.definition.key),
                        action: GuiAction {
                            command: binding.command,
                            active: true,
                            layer_id: 0,
                        },
                    });
                }
            }
        }
        self.modes.push(AppMode {
            name: "Default".to_string(),
            hotkeys,
        });
        self.selected_mode = 0;
        Ok(())
    }

  pub fn save_to_swhkd_config(&mut self) -> Result<(), String> {
    let mut seen = HashSet::new();
    for mode in &self.modes {
        for hk in &mode.hotkeys {
            if hk.action.active {
                let sig = (hk.modifiers.clone(), hk.key.clone());
                if !seen.insert(sig) {
                    return Err(format!(
                        "Duplicate binding: {} + {}",
                        hk.modifiers.iter().cloned().collect::<Vec<_>>().join(" + "),
                        hk.key
                    ));
                }
            }
        }
    }

    let config_path = self.get_swhkd_config_path().map_err(|e| e.to_string())?;
    if let Some(parent) = config_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if config_path.exists() {
        let backup_path = config_path.with_extension("bak");
        fs::copy(&config_path, &backup_path).map_err(|e| e.to_string())?;
        self.last_backup = Some(backup_path);
    }

    let mut config_text = String::new();

    for mode in &self.modes {
        // Write mode header only if mode name is not "Default" (case insensitive)
        if mode.name.to_lowercase() != "default" {
            config_text.push_str(&format!("@{}\n\n", mode.name));
        }

        for hk in &mode.hotkeys {
            if hk.action.active {
                // Sort modifiers for consistent output
                let mut mod_vec = hk.modifiers.iter().cloned().collect::<Vec<_>>();
                mod_vec.sort();

                let combo = if mod_vec.is_empty() {
                    hk.key.clone()
                } else {
                    format!("{} + {}", mod_vec.join(" + "), hk.key)
                };

                // Write one hotkey binding: combo line, indented command line and blank line between bindings
                config_text.push_str(&format!("{}\n    {}\n\n", combo, hk.action.command));
            }
        }
        config_text.push('\n'); // Extra blank line between modes for clarity
    }

    fs::write(&config_path, config_text).map_err(|e| e.to_string())?;
    let _ = self.reload_swhkd();
    Ok(())
}

    fn reload_swhkd(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = Command::new("pkill").arg("-USR1").arg("swhkd").output();
        let _ = Command::new("systemctl").args(["--user", "restart", "swhkd"]).output();
        Ok(())
    }
}

fn modifier_to_string(m: &Modifier) -> String {
    match m {
        Modifier::Control => "ctrl".to_string(),
        Modifier::Shift => "shift".to_string(),
        Modifier::Alt => "alt".to_string(),
        Modifier::Super => "super".to_string(),
        _ => format!("{:?}", m).to_lowercase(),
    }
}

fn string_to_modifier(s: &str) -> Option<Modifier> {
    match s.to_ascii_lowercase().as_str() {
        "ctrl" | "control" => Some(Modifier::Control),
        "alt" => Some(Modifier::Alt),
        "shift" => Some(Modifier::Shift),
        "super" => Some(Modifier::Super),
        _ => None,
    }
}

fn key_to_string(key: &Key) -> String {
    match key.key {
        EvdevKey::KEY_A => "a",
        EvdevKey::KEY_B => "b",
        EvdevKey::KEY_C => "c",
        EvdevKey::KEY_D => "d",
        EvdevKey::KEY_E => "e",
        EvdevKey::KEY_F => "f",
        EvdevKey::KEY_G => "g",
        EvdevKey::KEY_H => "h",
        EvdevKey::KEY_I => "i",
        EvdevKey::KEY_J => "j",
        EvdevKey::KEY_K => "k",
        EvdevKey::KEY_L => "l",
        EvdevKey::KEY_M => "m",
        EvdevKey::KEY_N => "n",
        EvdevKey::KEY_O => "o",
        EvdevKey::KEY_P => "p",
        EvdevKey::KEY_Q => "q",
        EvdevKey::KEY_R => "r",
        EvdevKey::KEY_S => "s",
        EvdevKey::KEY_T => "t",
        EvdevKey::KEY_U => "u",
        EvdevKey::KEY_V => "v",
        EvdevKey::KEY_W => "w",
        EvdevKey::KEY_X => "x",
        EvdevKey::KEY_Y => "y",
        EvdevKey::KEY_Z => "z",
        EvdevKey::KEY_1 => "1",
        EvdevKey::KEY_2 => "2",
        EvdevKey::KEY_3 => "3",
        EvdevKey::KEY_4 => "4",
        EvdevKey::KEY_5 => "5",
        EvdevKey::KEY_6 => "6",
        EvdevKey::KEY_7 => "7",
        EvdevKey::KEY_8 => "8",
        EvdevKey::KEY_9 => "9",
        EvdevKey::KEY_0 => "0",
        EvdevKey::KEY_ESC => "escape",
        EvdevKey::KEY_ENTER => "enter",
        EvdevKey::KEY_TAB => "tab",
        EvdevKey::KEY_SPACE => "space",
        EvdevKey::KEY_MINUS => "minus",
        EvdevKey::KEY_EQUAL => "equal",
        EvdevKey::KEY_BACKSLASH => "backslash",
        EvdevKey::KEY_LEFTBRACE => "bracketleft",
        EvdevKey::KEY_RIGHTBRACE => "bracketright",
        EvdevKey::KEY_SEMICOLON => "semicolon",
        EvdevKey::KEY_APOSTROPHE => "apostrophe",
        EvdevKey::KEY_COMMA => "comma",
        EvdevKey::KEY_DOT => "dot",
        EvdevKey::KEY_SLASH => "slash",
        _ => {
            // fallback: format to string
            return format!("{:?}", key.key);
        }
    }
    .to_string()
}

fn string_to_evdev_key(s: &str) -> Result<EvdevKey, String> {
    match s.to_ascii_lowercase().as_str() {
        "a" => Ok(EvdevKey::KEY_A),
        "b" => Ok(EvdevKey::KEY_B),
        "c" => Ok(EvdevKey::KEY_C),
        "d" => Ok(EvdevKey::KEY_D),
        "e" => Ok(EvdevKey::KEY_E),
        "f" => Ok(EvdevKey::KEY_F),
        "g" => Ok(EvdevKey::KEY_G),
        "h" => Ok(EvdevKey::KEY_H),
        "i" => Ok(EvdevKey::KEY_I),
        "j" => Ok(EvdevKey::KEY_J),
        "k" => Ok(EvdevKey::KEY_K),
        "l" => Ok(EvdevKey::KEY_L),
        "m" => Ok(EvdevKey::KEY_M),
        "n" => Ok(EvdevKey::KEY_N),
        "o" => Ok(EvdevKey::KEY_O),
        "p" => Ok(EvdevKey::KEY_P),
        "q" => Ok(EvdevKey::KEY_Q),
        "r" => Ok(EvdevKey::KEY_R),
        "s" => Ok(EvdevKey::KEY_S),
        "t" => Ok(EvdevKey::KEY_T),
        "u" => Ok(EvdevKey::KEY_U),
        "v" => Ok(EvdevKey::KEY_V),
        "w" => Ok(EvdevKey::KEY_W),
        "x" => Ok(EvdevKey::KEY_X),
        "y" => Ok(EvdevKey::KEY_Y),
        "z" => Ok(EvdevKey::KEY_Z),
        "1" => Ok(EvdevKey::KEY_1),
        "2" => Ok(EvdevKey::KEY_2),
        "3" => Ok(EvdevKey::KEY_3),
        "4" => Ok(EvdevKey::KEY_4),
        "5" => Ok(EvdevKey::KEY_5),
        "6" => Ok(EvdevKey::KEY_6),
        "7" => Ok(EvdevKey::KEY_7),
        "8" => Ok(EvdevKey::KEY_8),
        "9" => Ok(EvdevKey::KEY_9),
        "0" => Ok(EvdevKey::KEY_0),
        "escape" => Ok(EvdevKey::KEY_ESC),
        "enter" => Ok(EvdevKey::KEY_ENTER),
        "tab" => Ok(EvdevKey::KEY_TAB),
        "space" => Ok(EvdevKey::KEY_SPACE),
        "minus" => Ok(EvdevKey::KEY_MINUS),
        "equal" => Ok(EvdevKey::KEY_EQUAL),
        "backslash" => Ok(EvdevKey::KEY_BACKSLASH),
        "bracketleft" => Ok(EvdevKey::KEY_LEFTBRACE),
        "bracketright" => Ok(EvdevKey::KEY_RIGHTBRACE),
        "semicolon" => Ok(EvdevKey::KEY_SEMICOLON),
        "apostrophe" => Ok(EvdevKey::KEY_APOSTROPHE),
        "comma" => Ok(EvdevKey::KEY_COMMA),
        "dot" => Ok(EvdevKey::KEY_DOT),
        "slash" => Ok(EvdevKey::KEY_SLASH),
        _ => Err(format!("Unsupported key: {}", s)),
    }
}
