use iced::{
    Element,
    widget::{button, column},
};

use crate::Message;

//msg: Element<'a, Message>
pub fn nav_bar<'a>() -> Element<'a, Message> {
    column![
        button("Library").on_press(Message::ButtonHome),
        button("Plugins").on_press(Message::ButtonHome),
        button("Settings").on_press(Message::ButtonHome),
    ]
    .spacing(10)
    .into()
}
