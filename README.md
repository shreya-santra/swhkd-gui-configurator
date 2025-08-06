# SWHKD GUI – Visual Hotkey Manager for SWHKD 

A graphical user interface to configure hotkeys and modes for Simple Wayland Hotkey Daemon (SWHKD) without editing configuration files by hand . This application gives users full control over hotkey commands,modes and activation states. It uses [Sweet](https://github.com/waycrate/sweet) parser for parsing and serialization of the .swhkdrc configuration. Users can add, edit, remove, or toggle hotkeys and modes. It is written in Rust using [Iced](https://github.com/iced-rs/iced) -rs.

## Key Features 

- Mode-Based Organization: Users can control several hotkey modes (such as "Default," "Game Mode," etc.) visually.

- Instant Key Capture: The GUI records key combinations directly from the keyboard when users press them, allowing users to add or modify hotkeys seamlessly within the graphical user interface and reduces the chances of errors. 

- Instantaneous Configuration Write: Any modifications are instantly reflected in the user's live SWHKD configuration file (~/.config/swhkd/swhkdrc).

- Load Existing Configurations: On uploading the current SWHKD config file , the GUI will automatically read and import all existing hotkeys.

- Conflict Detection: The app automatically detects and warns about duplicate or conflicting hotkey assignments.

## Installation 

<pre>git clone https://github.com/shreya-santra/swhkd-gui-configurator.git
cd swhkd-gui-configurator
cargo build --release
cargo run --release</pre>

## Configuration

The GUI interacts directly with SWHKD’s live configuration file, typically located at ~/.config/swhkd/swhkdrc. The syntax is compatible with swhkd, so users can load and modify existing swhkd configs without extra steps.




