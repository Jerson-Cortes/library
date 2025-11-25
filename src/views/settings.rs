use iced::{
    widget::{ container, row, toggler},
    Element, Length, 
};

use crate::{Library,Message};

fn decorations<'a>(data: &Library) -> Element<'a, Message> {
    row![
        container("Decorations").center_x(Length::Fill),
        container(
            toggler(data.window_settings.decorations)
                .on_toggle(Message::DecorationsToggle)
        ).center_x(Length::Fill)
    ]
        .spacing(10)
        .into()
}

pub fn settings_screen<'a>(data: &Library) -> Element<'a, Message> {
    decorations(data)
}
