use iced::widget::{
    button, checkbox, column, container, row, scrollable, text, text_input, Space, image, image::Image,
};
use iced::{Alignment, Color, Element, Length, BorderRadius, Theme};
use iced::font::Weight;
use crate::data_model::AppState;

const BACKGROUND: Color = Color::WHITE;
const CARD: Color = Color::from_rgb(0.95, 0.96, 0.97);
const CARD_BORDER: Color = Color::from_rgb(0.87, 0.89, 0.91);
const ACCENT: Color = Color::from_rgb(0.40, 0.45, 0.53);
const ERROR: Color = Color::from_rgb(0.86, 0.21, 0.21);
const TEXT_PRIMARY: Color = Color::from_rgb(0.11, 0.11, 0.11);

pub struct CardContainer;
impl iced::widget::container::StyleSheet for CardContainer {
    type Style = Theme;
    fn appearance(&self, _: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(CARD.into()),
            border_radius: BorderRadius::from(12.0),
            border_width: 1.0,
            border_color: CARD_BORDER,
            ..Default::default()
        }
    }
}
pub struct CardButton;
impl button::StyleSheet for CardButton {
    type Style = Theme;
    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(CARD_BORDER.into()),
            border_radius: 8.0.into(),
            text_color: TEXT_PRIMARY,
            ..Default::default()
        }
    }
    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        let mut a = self.active(&Theme::default());
        a.background = Some(CARD.into());
        a
    }
}
pub struct DangerButton;
impl button::StyleSheet for DangerButton {
    type Style = Theme;
    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(CARD.into()),
            border_radius: 8.0.into(),
            border_width: 1.2,
            border_color: ERROR,
            text_color: ERROR,
            ..Default::default()
        }
    }
    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        let mut a = self.active(&Theme::default());
        a.background = Some(iced::Background::Color(Color { r: ERROR.r, g: ERROR.g, b: ERROR.b, a: 0.08 }));
        a
    }
}
pub struct PastelTextInput;
impl text_input::StyleSheet for PastelTextInput {
    type Style = Theme;
    fn active(&self, _: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: CARD.into(),
            border_radius: 8.0.into(),
            border_width: 1.0,
            border_color: CARD_BORDER,
            icon_color: CARD_BORDER,
        }
    }
    fn focused(&self, _: &Self::Style) -> text_input::Appearance {
        let mut a = self.active(&Theme::default());
        a.border_color = ACCENT;
        a
    }
    fn placeholder_color(&self, _: &Self::Style) -> Color { CARD_BORDER }
    fn value_color(&self, _: &Self::Style) -> Color { TEXT_PRIMARY }
    fn disabled_color(&self, _: &Self::Style) -> Color { CARD_BORDER }
    fn selection_color(&self, _: &Self::Style) -> Color { ACCENT }
    fn disabled(&self, _: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Color::from_rgb(0.96, 0.96, 0.96).into(),
            border_radius: 8.0.into(),
            border_width: 1.0,
            border_color: CARD_BORDER,
            icon_color: CARD_BORDER,
        }
    }
}

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
        let trash_icon = Image::new("assets/icons8-delete-30.png")
            .width(Length::Fixed(20.0))
            .height(Length::Fixed(20.0));
        let delete_button = button(trash_icon)
            .on_press(Message::DeleteMode(i))
            .style(iced::theme::Button::Custom(Box::new(DangerButton)))
            .padding(4)
            .width(Length::Fixed(34.0))
            .height(Length::Fixed(34.0));
        let mode_button = button(
            text(&mode.name)
                .style(TEXT_PRIMARY)
                .size(18),
        )
        .on_press(Message::SelectMode(i))
        .width(Length::FillPortion(1))
        .padding(9)
        .style(iced::theme::Button::Custom(Box::new(CardButton)));
        let mode_row = row![mode_button, delete_button]
            .spacing(7)
            .align_items(Alignment::Center);
        mode_list = mode_list.push(mode_row).push(Space::with_height(Length::Fixed(6.0))); 
    }
    mode_list = mode_list.push(
        button(
            text("Add Mode").style(TEXT_PRIMARY).size(16),
        )
        .on_press(Message::AddMode)
        .width(Length::Fill)
        .padding(9)
        .style(iced::theme::Button::Custom(Box::new(CardButton)))
    );
   let mode_list = container(
    column![
        
        container(
            text("Modes")
                .size(21)
                .style(TEXT_PRIMARY)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
        )
        .width(Length::Fill)
        .center_x(),
        Space::with_height(Length::Fixed(24.0)), // <-- Add gap here
        scrollable(mode_list).height(Length::Fill).width(Length::Fill),
    ]
)
.padding(20)
.width(Length::FillPortion(1))
.style(iced::theme::Container::Custom(Box::new(CardContainer)));


    let selected_mode = state
        .modes
        .get(state.selected_mode)
        .unwrap_or(&state.modes[0]);

   
    let mode_name_section = container(
        row![
            text("Mode:")
                .size(17)
                .style(TEXT_PRIMARY)
                .vertical_alignment(iced::alignment::Vertical::Center),
            Space::with_width(Length::Fixed(8.0)),
            text_input("Mode name", &selected_mode.name)
                .on_input(Message::EditModeName)
                .padding(8)
                .size(17)
                .style(iced::theme::TextInput::Custom(Box::new(PastelTextInput)))
                .width(Length::Fixed(180.0))
                .style(iced::theme::TextInput::Custom(Box::new(PastelTextInput))),
        ]
        .align_items(Alignment::Center)
        .spacing(8),
    )
    .padding(12)
    .style(iced::theme::Container::Custom(Box::new(CardContainer)));

    let header_row: iced::widget::Container<'_, Message, iced::Renderer<Theme>> = container(
        row![
            text("Key Combination").width(Length::FillPortion(3)).size(15).style(TEXT_PRIMARY).horizontal_alignment(iced::alignment::Horizontal::Center),
            text("Key Combination").width(Length::FillPortion(3)).size(15).style(TEXT_PRIMARY).horizontal_alignment(iced::alignment::Horizontal::Center),
            text("Command").width(Length::FillPortion(4)).size(15).style(TEXT_PRIMARY).horizontal_alignment(iced::alignment::Horizontal::Center),
            text("Active").width(Length::Fixed(90.0)).size(15).style(TEXT_PRIMARY).horizontal_alignment(iced::alignment::Horizontal::Center),
            text("Delete").width(Length::Fixed(90.0)).size(15).horizontal_alignment(iced::alignment::Horizontal::Center),
            text("Record").width(Length::Fixed(90.0)).size(15).horizontal_alignment(iced::alignment::Horizontal::Center),
        ]
        .spacing(8)
        .align_items(Alignment::Center)
    )
    .padding([8, 3, 8, 8])
    .style(iced::theme::Container::Custom(Box::new(CardContainer)));

    //Hotkey Rows 
    let mut hotkey_rows = column![];
    for (i, hk) in selected_mode.hotkeys.iter().enumerate() {
        let recording = state.recording_hotkey == Some(i);
        let key_display = if hk.key.is_empty() && recording {
            "Press a key combination...".to_string()
        } else if hk.modifiers.is_empty() {
            hk.key.clone()
        } else {
            format!(
                "{} + {}",
                hk.modifiers.iter().cloned().collect::<Vec<_>>().join(" + "),
                hk.key
            )
        };
        let key_cell = container(
            text(key_display)
                .size(17)
                .style(TEXT_PRIMARY)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
        )
        .padding([12, 0, 12, 0])
        .width(Length::FillPortion(3))
        .center_x()
        .center_y()
        .style(iced::theme::Container::Custom(Box::new(CardContainer)));

        let file_picker_icon = Image::new("assets/icons8-file-explorer-64.png")
            .width(Length::Fixed(20.0))
            .height(Length::Fixed(20.0));
        let file_picker_button = button(file_picker_icon)
            .on_press(Message::OpenBinaryPicker(i))
            .padding(6)
            .style(iced::theme::Button::Custom(Box::new(CardButton)));

        let record_btn = button(text("Record").size(14))
            .on_press(Message::StartRecording(i))
            .padding(5)
            .style(iced::theme::Button::Custom(Box::new(CardButton)))
            .width(Length::Fixed(90.0));

        let command_cell = row![
            text_input("Command", &hk.action.command)
                .on_input(move |val| Message::EditCommand(i, val))
                .width(Length::FillPortion(3))
                .padding(8)
                .style(iced::theme::TextInput::Custom(Box::new(PastelTextInput))),
            file_picker_button
        ]
        .spacing(10)
        .width(Length::FillPortion(4));

        
        let active_box = container(
            checkbox("", hk.action.active, move |checked| Message::ToggleActive(i, checked))
                .size(24)
        )
        .width(Length::Fixed(90.0))
        .center_x()
        .center_y();

        
        let delete_btn = button(
        text("DELETE")
            .style(TEXT_PRIMARY)
            .size(15)
    )
    .on_press(Message::DeleteHotkey(i))
    .style(iced::theme::Button::Custom(Box::new(DangerButton)))
    .padding([7,7])
    .width(Length::Fixed(90.0));

        let hotkey_row = row![
            key_cell,
            command_cell,
            active_box,
            delete_btn,
            record_btn,
        ]
        .spacing(8)
        .align_items(Alignment::Center);
        hotkey_rows = hotkey_rows
            .push(hotkey_row)
            .push(Space::with_height(Length::Fixed(10.0)));
    }

     
    let controls = container(
        row![
            button(text("Add Hotkey").style(TEXT_PRIMARY))
                .on_press(Message::AddHotkey)
                .padding(14)
                .style(iced::theme::Button::Custom(Box::new(CardButton)))
                .width(Length::FillPortion(1)),
            button(text("Load Config").style(TEXT_PRIMARY))
                .on_press(Message::LoadConfigFile)
                .padding(14)
                .style(iced::theme::Button::Custom(Box::new(CardButton)))
                .width(Length::FillPortion(1)),
            button(text("Save As...").style(TEXT_PRIMARY))
                .on_press(Message::SaveConfigAs)
                .padding(14)
                .style(iced::theme::Button::Custom(Box::new(CardButton)))
                .width(Length::FillPortion(1)),
            button(text("Save & Apply").style(TEXT_PRIMARY))
                .on_press(Message::SaveConfig)
                .padding(14)
                .style(iced::theme::Button::Custom(Box::new(CardButton)))
                .width(Length::FillPortion(1)),
        ]
    )
    .padding([16, 0, 0, 0])
    .width(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(CardContainer)));

    let error_text: Element<'_, Message> = if let Some(msg) = error {
            text(msg)
                .style(ERROR)
                .size(16)
                .into()
        } else {
            Space::with_height(Length::Fixed(0.0)).into()
        };

    let right_panel = column![
        mode_name_section,
        Space::with_height(Length::Fixed(24.0)),
        text("Hotkey Configuration")
            .size(21)
            .style(TEXT_PRIMARY),
            Space::with_height(Length::Fixed(20.0)), // or 24.0, 32.0 etc. as desired

        error_text,
        scrollable(hotkey_rows),
        controls,
    ]
    .width(Length::FillPortion(3))
    .padding(25) ; 
    let full_row = row![
        mode_list,
        Space::with_width(Length::Fixed(20.0)),
        right_panel,
    ]
    .width(Length::Fill)
    .height(Length::Fill);

    container(full_row)
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(iced::theme::Container::Transparent)
        .style(iced::theme::Container::Transparent)
        .into()
}
