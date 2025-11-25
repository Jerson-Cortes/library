use iced::{
    widget::{column, container, row, text, button},
    Element, Length,
};

use crate::Message;

#[derive(Default)]
pub struct HomeScreen {
}

impl HomeScreen {

    fn welcome_message<'a>() -> Element<'a, Message> {
        row![text!("Welcome to your E‑book Library!")]
            .spacing(10)
            .into()
    }

    fn debug_button<'a>() -> Element<'a, Message> {
        row![button("debug").on_press(Message::DebugToggle)]
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

    pub fn home<'a>() -> Element<'a, Message> {
        let msg = HomeScreen::welcome_message();
        let btn = HomeScreen::debug_button();
        HomeScreen::main_column(msg, btn)
    }
}
