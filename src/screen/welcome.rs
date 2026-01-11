use iced::{Element, Length, widget::{text, column, container, row}};

use crate::Message;

fn welcome_message<'a>() -> Element<'a, Message> {
    row![text!("Welcome to your E‑book Library!")]
        .spacing(10)
        .into()
}

fn main_column<'a>(msg: Element<'a, Message>, btn: Element<'a, Message>) -> Element<'a, Message> {
    column![
        container(msg).center_x(Length::Fill),
        container(btn).center_x(Length::Fill)
    ]
        .spacing(10)
        .into()
}

fn debug_button<'a>() -> Element<'a, Message> {
    row![text!("Place Holder")]
        .spacing(10)
        .into()
}

pub fn welcome_screen<'a>() -> Element<'a, Message> {
    main_column(welcome_message(),debug_button())
}
