use iced::{
    Element,
};
use iced::{
    widget,
};

pub fn main() -> iced::Result {
    iced::application(Library::default, Library::update, Library::view)
        .title(Library::title)
        .centered()
        .run()
}

pub struct Library {
    screen: Screen,
}

#[derive(Debug, Clone)]
pub enum Message {
    DebugToggle
}

impl Library {
    fn title(&self) -> String {
        let screen = match self.screen {
            Screen::Home => "Welcome",
        };

        format!("{screen} - Library")
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::DebugToggle => {
            }
        }
    }

    fn view(&self)  -> Element<'_, Message> {
        let row = widget::row![
            widget::text!("Welcome to your E-book Library!"),
            widget::button("-").on_press(Message::DebugToggle),
        ]
            .spacing(10);
        widget::container(row)
            .center_x(iced::Length::Fill)
            .center_y(iced::Length::Fill)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Home
}

impl Screen {

}

impl Default for Library {
    fn default() -> Self {
        Self {
            screen: Screen::Home,
        }
    }
}
