use iced::{
    Color, Element
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
    debug: bool,
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
                self.debug = !self.debug
            }
        }
    }

    fn view(&self)  -> Element<'_, Message> {

        let message: Element<_> = widget::row![
            widget::text!("Welcome to your E-book Library!")
        ]
            .spacing(10).into();

        let button: Element<_> = widget::row![
            widget::button("-").on_press(Message::DebugToggle)
        ]
            .spacing(10).into();

        let col: Element<_> = widget::column![
            widget::container(message).center_x(iced::Length::Fill),
            widget::container(button).center_x(iced::Length::Fill)
        ]
            .spacing(10).into();

        let wrapper: Element<_> = widget::container(col)
            .center_x(iced::Length::Fill)
            .center_y(iced::Length::Fill)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into();

        if self.debug {
            wrapper.explain(Color::BLACK)
        } else {
            wrapper
        }
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
            debug: false,
        }
    }
}
