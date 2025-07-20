use sweet_git::{Binding, Key, Modifier, Command};
use std::path::PathBuf;
use std::fs;
#[derive(Debug, Clone)]
pub struct GuiBinding {
    pub modifiers: Vec<Modifier>,
    pub key: Key,
    pub command: String,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub original: Vec<GuiBinding>,
    pub working: Vec<GuiBinding>,
    pub last_backup: Option<PathBuf>,
}
impl AppState {
    pub fn new() -> Self {
        Self {
            original: vec![],
            working: vec![],
            last_backup: None,
        }
    }

    
    pub fn load_from_swhkd_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path();
        if config_path.exists() {
            let text = fs::read_to_string(&config_path)?;
            let parsed = sweet_git::parse_config(&text)?;
            let bindings = parsed.bindings.into_iter().map(|b| GuiBinding {
                modifiers: b.definition.mods,
                key: b.definition.key,
                command: match b.definition.command {
                    Command::Simple(s) => s,
                    _ => String::new(),
                },
                active: true,
            }).collect();
            self.original = bindings.clone();
            self.working = bindings;
        }
        Ok(())
    }

    pub fn save_to_swhkd_config(&mut self) -> Result<(), String> {
        if let Some(dup) = find_duplicate(&self.working) {
            return Err(format!("Duplicate keybinding detected: {dup}"));
        }

        let config_path = Self::config_path();
        if config_path.exists() {
            let backup_path = config_path.with_extension("bak");
            fs::copy(&config_path, &backup_path)
                .map_err(|e| format!("Failed to backup: {e}"))?;
            self.last_backup = Some(backup_path);
        }

        let bindings: Vec<Binding> = self.working.iter().map(|g| Binding {
            definition: sweet_git::HotkeyDefinition {
                mods: g.modifiers.clone(),
                key: g.key.clone(),
                command: Command::Simple(g.command.clone()),
            },
            mode_instructions: Vec::new(),
        }).collect();

        let config = sweet_git::Config { bindings };
        let config_text = config.to_string();
        fs::create_dir_all(config_path.parent().unwrap())
            .map_err(|e| format!("Failed to create config dir: {e}"))?;
        fs::write(&config_path, config_text)
            .map_err(|e| format!("Failed to write config: {e}"))?;
        self.original = self.working.clone();
        Ok(())
    }

    fn config_path() -> PathBuf {
        dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("swhkd")
            .join("swhkdrc")
    }
}

fn find_duplicate(bindings: &[GuiBinding]) -> Option<String> {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    for b in bindings {
        let key = (
            b.modifiers.iter().map(|m| format!("{m:?}")).collect::<Vec<_>>(),
            format!("{:?}", b.key),
        );
        if !seen.insert(key) {
            return Some(format!("{:?} + {:?}", b.modifiers, b.key));
        }
    }
    None
}
