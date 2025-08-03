# SWHKD GUI – Visual Hotkey Manager for Wayland

A modern graphical interface for [SWHKD](https://github.com/waycrate/swhkd), enabling you to visually manage keyboard shortcuts and modes *directly* in your live SWHKD config file—no manual text editing required . 

## ✨ Features

- **Multiple Modes:** Create, rename, and switch between keyboard profile "modes" (e.g., Default, Game Mode).
- **Visual Hotkey Editing:** Add, delete, and edit hotkeys with immediate effect.
- **Record Key Combos:** Capture any key combination live from the GUI.
- **Toggle On/Off:** Activate or deactivate any hotkey by checkbox.
- **File Picker:** Quickly select executable/script paths as commands.
- **Zero-handoff Workflow:** All edits are **immediately written to your system’s SWHKD config file** (typically `~/.config/swhkd/swhkdrc`).
- **Sample configs and tests** for parser robustness.


## 💾 How Configuration Works

**Every change made in the GUI is instantly applied to the actual SWHKD config file—by default:**

There is **no need to manually export, import, or copy files** after editing: as soon as you hit “Add Hotkey”, “Delete”, “Save”, etc., the config file updates atomically behind the scenes.  
Edits are immediately visible from the GUI *and* your terminal text editor.

## 🗂️ Directory Structure

```
assets/                       # Icons and GUI assets
  ├── icons8-delete-30.png
  ├── icons8-file-explorer-64.png
src/                          # Rust source (data models, interface, main logic)
  ├── data_model.rs
  ├── interface.rs
  ├── key_recording.rs
  ├── main.rs
tests/
  ├── parser_loading.rs       # Config parser test suite
  ├── sweet_samples/          # Sample configs for CI/testing
basic_keybind.skwhrc          # Example CLI config
Cargo.toml                    # Build specification
README.md                     # This file
```

## 🛠️ Build & Run

**Requirements:**
- [Rust](https://www.rust-lang.org/) (stable)
- Wayland compositor (Sway, Hyprland, etc.)
- SWHKD running on your system

**To build and run:**
```bash
git clone https://github.com/yourusername/swhkd-gui.git
cd swhkd-gui
cargo build --release
cargo run --release
```
This launches the GUI. Your existing SWHKD config will update live as you edit.


## 🔬 Testing & Samples

- Sample configs for parser validation in `/tests/sweet_samples/`
- Confirm correct config rendering for different mode/hotkey scenarios


## 📝 License

Dual-licensed under MIT and Apache-2.0

**Developed by Shreya Santra as part of Google Summer of Code 2025, with mentorship from the [Waycrate](https://waycrate.github.io) organization.**

**All changes you make in this app are instantly and automatically reflected in your live SWHKD config file—no manual file handling needed!**

---
