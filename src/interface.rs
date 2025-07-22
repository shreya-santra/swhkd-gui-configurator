use iced::widget::{button, checkbox, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Element, Length, Theme};
use crate::data_model::{AppState , AppMode, GuiHotkey};
use sweet::{Key, Modifier};

#[derive(Debug, Clone)]
pub enum Message {
    SelectMode(usize),
    EditModeName(String),
    EditKey(usize, String),
    EditCommand(usize, String),
    ToggleActive(usize, bool),
    DeleteHotkey(usize),
    AddHotkey,
    StartRecording(usize),
    KeyRecorded(String),
    StopRecording,
    AddMode,
    SaveConfig,
    LoadConfig,
    ShowError(String),
    ClearError,
}

pub fn view<'a>(state: &'a AppState, error: &'a Option<String>) -> Element<'a, Message> {
    if state.modes.is_empty() {
        return container(
            text("No apps available. Please add a mode.")
        )
        .center_x()
        .center_y()
        .padding(40)
        .into();
    }

    // Left panel: Mode list (scrollable)
    let mut mode_list = column![];
    for (i, mode) in state.modes.iter().enumerate() {
        let btn = button(text(&mode.name))
            .on_press(Message::SelectMode(i))
            .width(Length::Fill)
            .padding(12);
        let btn = if i == state.selected_mode {
            btn.style(iced::theme::Button::Primary)
        } else {
            btn
        };
        mode_list = mode_list.push(btn);
    }
    mode_list = mode_list.push(Space::with_height(Length::Fixed(15.0)));
    mode_list = mode_list.push(
        button(text("+ Add Mode"))
            .on_press(Message::AddMode)
            .width(Length::Fill)
            .padding(12)
            .style(iced::theme::Button::Secondary)
    );
    let mode_list = container(
        scrollable(mode_list)
            .height(Length::Fill)
            .width(Length::Fill)
    )
    .padding(20);

    let selected_mode = &state.modes[state.selected_mode];

    let mode_name_section = container(
        column![
            text("Mode Settings").size(20),
            Space::with_height(Length::Fixed(10.0)),
            text_input("Mode Name", &selected_mode.name)
                .on_input(Message::EditModeName)
                .padding(8)
                .size(18)
        ]
        .spacing(8)
    )
    .padding(20)
    .style(iced::theme::Container::Box);

    let header_row = container(
        row![
            text("Key Combination").width(Length::FillPortion(3)),
            text("Command").width(Length::FillPortion(4)),
            text("Active").width(Length::FillPortion(1)),
            text("Delete").width(Length::FillPortion(1)),
            text("Record").width(Length::FillPortion(1)),
        ]
        .spacing(20)
        .align_items(Alignment::Center)
    )
    .padding(15)
    .style(iced::theme::Container::Box);

    let mut hotkey_rows = column![];
    for (i, hotkey) in selected_mode.hotkeys.iter().enumerate() {
        let key_display = if hotkey.modifiers.is_empty() {
            hotkey.key.clone()
        } else {
            format!(
                "{} + {}",
                hotkey.modifiers.iter().cloned().collect::<Vec<_>>().join(" + "),
                hotkey.key
            )
        };
        let recording_indicator = if state.recording_hotkey == Some(i) {
            "🔴"
        } else {
            "⎈"
        };
        let row_elem = container(
            row![
                text_input("Key Combination", &key_display)
                    .on_input(move |val| Message::EditKey(i, val))
                    .width(Length::FillPortion(3))
                    .padding(8),
                text_input("Command", &hotkey.action.command)
                    .on_input(move |val| Message::EditCommand(i, val))
                    .width(Length::FillPortion(4))
                    .padding(8),
                container(
                    checkbox(
                        "",
                        hotkey.action.active,
                        move |is_checked| Message::ToggleActive(i, is_checked),
                    )
                )
                .width(Length::FillPortion(1))
                .center_x(),
                container(
                    button(text("Delete"))
                        .on_press(Message::DeleteHotkey(i))
                        .style(iced::theme::Button::Destructive)
                        .padding(8)
                )
                .width(Length::FillPortion(1)),
                container(
                    button(text(recording_indicator))
                        .on_press(Message::StartRecording(i))
                        .padding(8)
                )
                .width(Length::FillPortion(1)),
            ]
            .spacing(20)
            .align_items(Alignment::Center)
        )
        .padding(12)
        .style(iced::theme::Container::Box);
        hotkey_rows = hotkey_rows.push(row_elem);
        hotkey_rows = hotkey_rows.push(Space::with_height(Length::Fixed(8.0)));
    }

    let controls = container(
        row![
            button(text("Add Hotkey"))
                .on_press(Message::AddHotkey)
                .padding(12)
                .style(iced::theme::Button::Primary),
            Space::with_width(Length::Fixed(20.0)),
            button(text("💾 Save & Apply"))
                .on_press(Message::SaveConfig)
                .padding(12)
                .style(iced::theme::Button::Primary),
            button(text("⟳ Load"))
                .on_press(Message::LoadConfig)
                .padding(12)
                .style(iced::theme::Button::Secondary),
        ]
        .spacing(15)
    )
    .padding(20);

    let error_text = if let Some(msg) = error {
        text(msg).style(iced::theme::Text::Color(iced::Color::from_rgb(1.0, 0.0, 0.0)))
    } else {
        text("")
    };

    let right_panel = column![
        mode_name_section,
        Space::with_height(Length::Fixed(20.0)),
        text("Hotkey Configuration").size(20),
        Space::with_height(Length::Fixed(15.0)),
        header_row,
        Space::with_height(Length::Fixed(10.0)),
        scrollable(hotkey_rows).height(Length::Fill),
        Space::with_height(Length::Fixed(20.0)),
        controls,
        error_text,
    ]
    .spacing(0)
    .width(Length::Fill);

    container(
        row![
            container(
                column![
                    container(text("MODES").size(22))
                        .padding(20)
                        .style(iced::theme::Container::Box),
                    Space::with_height(Length::Fixed(15.0)),
                    mode_list
                ]
            )
            .width(Length::FillPortion(1)),
            container(Space::with_width(Length::Fixed(2.0)))
                .style(iced::theme::Container::Box)
                .height(Length::Fill),
            container(right_panel)
                .width(Length::FillPortion(3))
                .padding(25),
        ]
        .spacing(0)
        .height(Length::Fill)
    )
    .padding(25)
    .height(Length::Fill)
    .into()
}

