use sweet::{Key, Modifier};
use evdev::Key as EvdevKey;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::fs::File;
use std::io::Write;



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
    
    pub fn load_swhkd_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = self.get_swhkd_config_path()?;
    self.modes.clear();

    let mut hotkeys = Vec::new();

    if config_path.exists() {
        let contents = fs::read_to_string(&config_path)?;

        let mut lines = contents.lines().peekable();

        while let Some(line) = lines.next() {
            let line = line.trim_end();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }


impl AppState {
    pub fn save_to_json_file(&self, path: &str) -> Result<(), String> {
        match serde_json::to_string_pretty(self) {
            Ok(json) => {
                File::create(path)
                    .and_then(|mut file| file.write_all(json.as_bytes()))
                    .map_err(|e| e.to_string())
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

            
            if line.starts_with('@') {
                continue; 
            }

            
            if !line.starts_with(' ') && !line.starts_with('\t') {
                let hotkey_line = line;

               
                let mut command_line = None;
                while let Some(next_line) = lines.peek() {
                    let trimmed = next_line.trim_end();

                    if trimmed.is_empty() || trimmed.starts_with('#') {
                        lines.next();
                        continue; 
                    }

                    if trimmed.starts_with(' ') || trimmed.starts_with('\t') {
                        command_line = Some(trimmed.trim());
                        lines.next(); 
                        break;
                    }

                    break;
                }

                if let Some(command) = command_line {
                    
                    let mut modifiers = BTreeSet::new();
                    let mut key = String::new();

                    let parts = hotkey_line.split('+').map(|s| s.trim().to_lowercase()).collect::<Vec<_>>();

                    if let Some(k) = parts.last() {
                        key = k.clone();
                        for m in &parts[0..parts.len().saturating_sub(1)] {
                            if !m.is_empty() {
                                modifiers.insert(m.clone());
                            }
                        }
                    }

                    hotkeys.push(GuiHotkey {
                        modifiers,
                        key,
                        action: GuiAction {
                            command: command.to_string(),
                            active: true,
                            layer_id: 0,
                        },
                    });
                }
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

pub fn load_from_json_file(path: &str) -> Self {
    use std::fs;
    if let Ok(json) = fs::read_to_string(path) {
        serde_json::from_str(&json).unwrap_or_default()
    } else {
        Self::default()
    }
}
pub fn new() -> Self {
    Self::default()
}

    pub fn get_swhkd_config_path(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let home = std::env::var("HOME").map_err(|_| "Could not determine $HOME")?;
        Ok(PathBuf::from(home).join(".config/swhkd/swhkdrc"))
    }

    pub fn load_from_swhkd_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = self.get_swhkd_config_path()?;
        self.modes.clear();

        let mut hotkeys = Vec::new();
        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            
            for line in contents.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                
                
                if let Some((combo, command)) = line.split_once('\n') {
                    let mut modifiers = BTreeSet::new();
                    let mut key = String::new();
                    
                    let mut combo_parts = combo.split('+').map(|s| s.trim().to_lowercase()).collect::<Vec<_>>();
                    if let Some(last) = combo_parts.pop() {
                        key = last;
                        for part in combo_parts {
                            modifiers.insert(part);
                        }
                    }
                    
                    hotkeys.push(GuiHotkey {
                        modifiers,
                        key,
                        action: GuiAction {
                            command: command.trim().to_string(),
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

    pub fn load_from_swhkd_config_at(&mut self, config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from(config_path.replace("$HOME", &std::env::var("HOME")?));
    if !path.exists() {
        return Err("File not found".into());
    }
    self.modes.clear();

    let contents = fs::read_to_string(&path)?;
    let mut lines = contents.lines().map(str::trim_end).peekable();

    let mut modes = Vec::new();
    let mut current_mode_name = "Default".to_string();
    let mut hotkeys = Vec::new();

    while let Some(line) = lines.next() {
        let line = line.trim_end();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(rest) = line.strip_prefix('@') {
            if !hotkeys.is_empty() || !modes.is_empty() {
                modes.push(AppMode {
                    name: std::mem::take(&mut current_mode_name),
                    hotkeys: std::mem::take(&mut hotkeys),
                });
            }
            current_mode_name = rest.trim().to_string();
            continue;
        }
        if !line.starts_with(' ') && !line.starts_with('\t') {
            let combo = line;
            let mut command_line = None;
            while let Some(next_line) = lines.peek() {
                let trimmed = next_line.trim_end();
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    lines.next();
                    continue;
                }
                if trimmed.starts_with(' ') || trimmed.starts_with('\t') {
                    command_line = Some(trimmed.trim());
                    lines.next();
                }
                break;
            }
            if let Some(command) = command_line {
                let mut modifiers = BTreeSet::new();
                let mut key = String::new();
                let combo_parts: Vec<_> = combo.split('+').map(|s| s.trim().to_lowercase()).collect();
                if let Some(last) = combo_parts.last() {
                    key = last.clone();
                    for part in &combo_parts[0..combo_parts.len().saturating_sub(1)] {
                        if !part.is_empty() {
                            modifiers.insert(part.clone());
                        }
                    }
                }
                hotkeys.push(GuiHotkey {
                    modifiers,
                    key,
                    action: GuiAction {
                        command: command.to_string(),
                        active: true,
                        layer_id: 0,
                    },
                });
            }
        }
    }
    if !hotkeys.is_empty() || modes.is_empty() {
        modes.push(AppMode {
            name: current_mode_name,
            hotkeys,
        });
    }

    self.modes = modes;
    self.selected_mode = 0;
    Ok(())
}




    
    pub fn save_to_custom_path(&self, path: &str) -> Result<(), String> {
        
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

        let config_path = PathBuf::from(path);

        if let Some(parent) = config_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let mut config_text = String::new();
        for mode in &self.modes {
            if mode.name.to_lowercase() != "default" {
                config_text.push_str(&format!("@{}\n\n", mode.name));
            }
            for hk in &mode.hotkeys {
                if hk.action.active {
                    let mut mod_vec = hk.modifiers.iter().cloned().collect::<Vec<_>>();
                    mod_vec.sort();
                    let combo = if mod_vec.is_empty() {
                        hk.key.clone()
                    } else {
                        format!("{} + {}", mod_vec.join(" + "), hk.key)
                    };
                    config_text.push_str(&format!("{}\n    {}\n\n", combo, hk.action.command));
                }
            }
            config_text.push('\n');
        }

        fs::write(&config_path, config_text).map_err(|e| e.to_string())?;
        Ok(())
    }

    
    pub fn save_to_swhkd_config(&self) -> Result<(), String> {
        let config_path = self.get_swhkd_config_path().map_err(|e| e.to_string())?;
        
        
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

        
        if let Some(parent) = config_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        
        let mut config_text = String::new();
        for mode in &self.modes {
            if mode.name.to_lowercase() != "default" {
                config_text.push_str(&format!("@{}\n\n", mode.name));
            }
            for hk in &mode.hotkeys {
                if hk.action.active {
                    let mut mod_vec = hk.modifiers.iter().cloned().collect::<Vec<_>>();
                    mod_vec.sort();
                    let combo = if mod_vec.is_empty() {
                        hk.key.clone()
                    } else {
                        format!("{} + {}", mod_vec.join(" + "), hk.key)
                    };
                    config_text.push_str(&format!("{}\n    {}\n\n", combo, hk.action.command));
                }
            }
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