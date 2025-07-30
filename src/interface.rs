use iced::widget::{button, checkbox, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Color, Element, Length};
use crate::data_model::AppState;
use iced::widget::{image, image::Image};

use iced::widget::Row; 


#[derive(Debug, Clone)]
pub enum Message {
    SelectMode(usize),
    EditModeName(String),
    EditCommand(usize, String),
    ToggleActive(usize, bool),
    DeleteHotkey(usize),
    DeleteMode(usize),
    AddHotkey,
    StartRecording(usize),
    KeyRecorded(String),
    StopRecording,
    AddMode,
    SaveConfig,
    ShowError(String),
    ClearError,
    OpenBinaryPicker(usize),
    BinaryPicked(usize, Option<String>),
    LoadConfigFile,
    ConfigFilePicked(Option<String>),
    SaveConfigAs,
    ConfigFileSavePath(Option<String>),

}

pub fn view<'a>(state: &'a AppState, error: &'a Option<String>) -> Element<'a, Message> {
    let mut mode_list = column![];
    for (i, mode) in state.modes.iter().enumerate() {
    let delete_icon = Image::new("assets/icons8-delete-30.png")
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0));

    let delete_button = button(delete_icon)
        .on_press(Message::DeleteMode(i))
        .style(iced::theme::Button::Destructive)
        .padding(4)
        .width(Length::Fixed(28.0))
        .height(Length::Fixed(28.0));

    let mode_button = button(text(&mode.name))
        .on_press(Message::SelectMode(i))
        .width(Length::FillPortion(1))
        .padding(8)
        .style(if i == state.selected_mode {
            iced::theme::Button::Primary
        } else {
            iced::theme::Button::Secondary
        });

    let mode_row = row![
        mode_button,
        delete_button
    ].spacing(6).align_items(Alignment::Center);

    mode_list = mode_list.push(mode_row).push(Space::with_height(Length::Fixed(2.0)));
}


    mode_list = mode_list.push(
        button(text("Add Mode"))
            .on_press(Message::AddMode)
            .width(Length::Fill)
            .padding(8)
            .style(iced::theme::Button::Primary),
    );
    let mode_list = container(
        column![
            text("Modes").size(18).style(iced::theme::Text::Color(Color::from_rgb(0.1, 0.3, 0.75))),
            scrollable(mode_list).height(Length::Fill).width(Length::Fill),
        ]
    )
    .padding(12)
    .width(Length::FillPortion(1))
    .style(iced::theme::Container::Box);

   let selected_mode = state
    .modes
    .get(state.selected_mode)
    .unwrap_or(&state.modes[0]);

    let mode_name_section = container(
        row![
            text("Mode Name: ").size(16),
            text_input("Mode Name", &selected_mode.name)
                .on_input(Message::EditModeName)
                .padding(6)
                .size(16)
                .width(Length::Fill)
        ].spacing(6)
    ).padding(10).style(iced::theme::Container::Box);

    let header_row = container(
        row![
            text("Key Combination").width(Length::FillPortion(3)).size(14),
            text("Command").width(Length::FillPortion(4)).size(14),
            text("Save Hotkey").width(Length::FillPortion(1)),
            text("Delete Hotkey").width(Length::FillPortion(1)),
            text("Record").width(Length::FillPortion(1)),
        ].spacing(8).align_items(Alignment::Center),
    )
    .padding([8, 3, 8, 8])
    .style(iced::theme::Container::Box);

    let mut hotkey_rows = column![];
    for (i, hk) in selected_mode.hotkeys.iter().enumerate() {
        let recording = state.recording_hotkey == Some(i);
        let key_display = if hk.key.is_empty() && recording {
            "Press a key combination...".to_string()
        } else {
            if hk.modifiers.is_empty() {
                hk.key.clone()
            } else {
                format!("{} + {}", hk.modifiers.iter().cloned().collect::<Vec<_>>().join(" + "), hk.key)
            }
        };
        let key_cell = if !hk.key.is_empty() || !recording {
            container(text(key_display)).padding(7).width(Length::FillPortion(3))
        } else {
            container(text(key_display).style(iced::theme::Text::Color(Color::from_rgb(0.8, 0.2, 0.2))))
                .padding(7)
                .width(Length::FillPortion(3))
        };
        let record_btn = container(button("Record")
            .on_press(Message::StartRecording(i))
            .padding(4)
            .style(if recording { iced::theme::Button::Primary } else { iced::theme::Button::Secondary }))
            .width(Length::FillPortion(1));

let folder_icon = Image::new("assets/icons8-file-explorer-64.png")
    .width(Length::Fixed(20.0))
    .height(Length::Fixed(20.0));

let file_picker_button = button(folder_icon)
    .on_press(Message::OpenBinaryPicker(i))
    .padding(7);

let command_cell = container(
    row![
        text_input("Command", &hk.action.command)
            .on_input(move |val| Message::EditCommand(i, val))
            .width(Length::FillPortion(3))
            .padding(7),
        file_picker_button,
    ]
    .spacing(8)
).width(Length::FillPortion(4));



        hotkey_rows = hotkey_rows.push(
            container(
                row![
                    key_cell,
                    command_cell,
                    checkbox("", hk.action.active, move |checked| Message::ToggleActive(i, checked))
                        .width(Length::FillPortion(1)),
                    button("Delete")
                        .on_press(Message::DeleteHotkey(i))
                        .style(iced::theme::Button::Destructive)
                        .padding(4)
                        .width(Length::FillPortion(1)),
                    record_btn,
                ]
                .spacing(8)
                .align_items(Alignment::Center),
            ).padding([8, 0])
             .style(iced::theme::Container::Box),
        );
        hotkey_rows = hotkey_rows.push(Space::with_height(Length::Fixed(6.0)));
    }
let controls = container(
    row![
        button(text("Add Hotkey"))
            .on_press(Message::AddHotkey)
            .padding(8)
            .style(iced::theme::Button::Primary),
        Space::with_width(Length::Fixed(12.0)),
        button(text("Load Config"))
            .on_press(Message::LoadConfigFile)
            .padding(8)
            .style(iced::theme::Button::Secondary),
        Space::with_width(Length::Fixed(12.0)),
        button(text("Save As..."))
            .on_press(Message::SaveConfigAs)
            .padding(8)
            .style(iced::theme::Button::Secondary),
        Space::with_width(Length::Fixed(12.0)),
        button(text("Save & Apply"))
            .on_press(Message::SaveConfig)
            .padding(8)
            .style(iced::theme::Button::Primary),
    ].spacing(12)
).padding(12);


    let error_text = if let Some(msg) = error {
        text(msg).style(iced::theme::Text::Color(Color::from_rgb(1.0, 0., 0.)))
    } else {
        text("")
    };

    let right_panel = column![
        mode_name_section,
        Space::with_height(Length::Fixed(8.0)),
        text("Hotkey Configuration").size(18).style(iced::theme::Text::Color(Color::from_rgb(0.1, 0.3, 0.75))),
        Space::with_height(Length::Fixed(4.0)),
        header_row,
        Space::with_height(Length::Fixed(4.0)),
        scrollable(hotkey_rows).height(Length::Fill),
        Space::with_height(Length::Fixed(4.0)),
        controls,
        error_text,
    ]
    .spacing(0)
    .width(Length::FillPortion(3))
    .padding(12);

    container(
        row![
            mode_list,
            container(Space::with_width(Length::Fixed(2.0))).style(iced::theme::Container::Box).height(Length::Fill),
            right_panel,
        ].spacing(0).height(Length::Fill),
    )
    .padding(0)
    .height(Length::Fill)
    .into()
}



