use iced::widget::{button, checkbox, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Element, Length};
use crate::data_model::AppState;

#[derive(Debug, Clone)]
pub enum Message {
    EditKey(usize, String),
    EditCommand(usize, String),
    ToggleActive(usize, bool),
    DeleteHotkey(usize),
    AddHotkey,
    SaveConfig,
    LoadConfig,
    ShowError(String),
    ClearError,
}

pub fn view<'a>(state: &'a AppState, error: &'a Option<String>) -> Element<'a, Message> {
    let mut hotkey_rows = column![];
    for (i, hotkey) in state.working.iter().enumerate() {
        let key_display = format!(
            "{} + {}",
            hotkey.modifiers.iter().map(|m| format!("{:?}", m)).collect::<Vec<_>>().join(" + "),
            format!("{:?}", hotkey.key)
        );
        let row_elem = row![
            text_input("Key Combination", &key_display)
                .on_input(move |val| Message::EditKey(i, val))
                .width(Length::FillPortion(3))
                .padding(8),
            text_input("Command", &hotkey.command)
                .on_input(move |val| Message::EditCommand(i, val))
                .width(Length::FillPortion(4))
                .padding(8),
            checkbox(
                "",
                hotkey.active
            )
            .on_toggle(move |is_checked| Message::ToggleActive(i, is_checked)),
            button(text("Delete"))
                .on_press(Message::DeleteHotkey(i))
                .style(iced::theme::Button::Destructive)
                .padding(8),
        ]
        .spacing(20)
        .align_items(Alignment::Center);
        hotkey_rows = hotkey_rows.push(row_elem);
        hotkey_rows = hotkey_rows.push(Space::with_height(Length::Fixed(8.0)));
    }

    let controls = row![
        button(text("Add Hotkey"))
            .on_press(Message::AddHotkey)
            .padding(12)
            .style(iced::theme::Button::Primary),
        button(text("💾 Save"))
            .on_press(Message::SaveConfig)
            .padding(12)
            .style(iced::theme::Button::Primary),
        button(text("⟳ Load"))
            .on_press(Message::LoadConfig)
            .padding(12)
            .style(iced::theme::Button::Secondary),
    ]
    .spacing(15);

    let error_text = if let Some(msg) = error {
        text(msg).style(iced::theme::Text::Color(iced::Color::from_rgba(1.0, 0.0, 0.0, 1.0)))
    } else {
        text("")
    };

    container(
        column![
            text("SWHKD GUI Hotkey Editor").size(28),
            scrollable(hotkey_rows).height(Length::Fill),
            controls,
            error_text,
        ]
        .spacing(16)
        .padding(20)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}
