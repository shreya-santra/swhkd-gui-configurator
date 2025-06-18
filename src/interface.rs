use iced::widget::{button, checkbox, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Element, Length};
use crate::data_model::{AppState, AppMode};

#[derive(Debug, Clone)]
pub enum Message {
    SelectApp(usize),
    EditAppName(String),
    EditKey(usize, String),
    EditCommand(usize, String),
    ToggleActive(usize, bool),
    DeleteHotkey(usize),
    AddHotkey,
    StartRecording(usize),
    KeyRecorded(String),
    StopRecording,
    AddApp,
    SaveConfig, // NEW: Save to SWHKD
}

pub fn view(state: &AppState) -> Element<Message> {
    if state.apps.is_empty() {
        return container(
            text("No apps available. Please add an app.")
        )
        .center_x()
        .center_y()
        .padding(40)
        .into();
    }

    // Left panel: App list (scrollable)
    let mut app_list = column![];
    for (i, app) in state.apps.iter().enumerate() {
        let btn = button(text(&app.name))
            .on_press(Message::SelectApp(i))
            .width(Length::Fill)
            .padding(12);
        let btn = if i == state.selected_app {
            btn.style(iced::theme::Button::Primary)
        } else {
            btn
        };
        app_list = app_list.push(btn);
    }
    app_list = app_list.push(Space::with_height(Length::Fixed(15.0)));
    app_list = app_list.push(
        button(text("+ Add App"))
            .on_press(Message::AddApp)
            .width(Length::Fill)
            .padding(12)
            .style(iced::theme::Button::Secondary)
    );
    let app_list = container(
        scrollable(app_list)
            .height(Length::Fill)
            .width(Length::Fill)
    )
    .padding(20);

    let selected_app: &AppMode = &state.apps[state.selected_app];

    let app_name_section = container(
        column![
            text("Application Settings").size(20),
            Space::with_height(Length::Fixed(10.0)),
            text_input("App Name", &selected_app.name)
                .on_input(Message::EditAppName)
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
    for (i, hotkey) in selected_app.hotkeys.iter().enumerate() {
        let key_display = if hotkey.modifiers.is_empty() {
            hotkey.key.clone()
        } else {
            format!("{} + {}", 
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
                        hotkey.action.active
                    )
                    .on_toggle(move |is_checked| Message::ToggleActive(i, is_checked))
                ).width(Length::FillPortion(1)).center_x(),
                container(
                    button(text("Delete"))
                        .on_press(Message::DeleteHotkey(i))
                        .style(iced::theme::Button::Destructive)
                        .padding(8)
                ).width(Length::FillPortion(1)),
                container(
                    button(text(recording_indicator))
                        .on_press(Message::StartRecording(i))
                        .padding(8)
                ).width(Length::FillPortion(1)),
            ]
            .spacing(20)
            .align_items(Alignment::Center)
        )
        .padding(12)
        .style(iced::theme::Container::Box);
        hotkey_rows = hotkey_rows.push(row_elem);
        hotkey_rows = hotkey_rows.push(Space::with_height(Length::Fixed(8.0)));
    }

    // Controls with Save button
    let controls = container(
        row![
            button(text("Add Hotkey"))
                .on_press(Message::AddHotkey)
                .padding(12)
                .style(iced::theme::Button::Primary),
            Space::with_width(Length::Fixed(20.0)),
            button(text("💾 Save & Apply to System"))
                .on_press(Message::SaveConfig)
                .padding(12)
                .style(iced::theme::Button::Primary)
        ]
        .spacing(15)
    )
    .padding(20);

    let right_panel = column![
        app_name_section,
        Space::with_height(Length::Fixed(20.0)),
        text("Hotkey Configuration").size(20),
        Space::with_height(Length::Fixed(15.0)),
        header_row,
        Space::with_height(Length::Fixed(10.0)),
        scrollable(hotkey_rows).height(Length::Fill),
        Space::with_height(Length::Fixed(20.0)),
        controls,
    ]
    .spacing(0)
    .width(Length::Fill);

    container(
        row![
            container(
                column![
                    container(text("APPS").size(22))
                        .padding(20)
                        .style(iced::theme::Container::Box),
                    Space::with_height(Length::Fixed(15.0)),
                    app_list
                ]
            ).width(Length::FillPortion(1)),
            container(Space::with_width(Length::Fixed(2.0)))
                .style(iced::theme::Container::Box)
                .height(Length::Fill),
            container(right_panel)
                .width(Length::FillPortion(3))
                .padding(25)
        ]
        .spacing(0)
        .height(Length::Fill)
    )
    .padding(25)
    .height(Length::Fill)
    .into()
}



