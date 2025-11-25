use iced::{
    Color, Element
};

use iced::{
    widget,
};

mod views;

use views::{
    home::HomeScreen
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
            Screen::Home => "Home",
            Screen::Settings => "Settings",
            Screen::Book => "Book",
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

        let screen = match self.screen {
            Screen::Home => HomeScreen::home(),
            Screen::Settings => HomeScreen::home(),
            Screen::Book => HomeScreen::home()
        };

        let content: Element<_> = widget::container(screen)
            .center_x(iced::Length::Fill)
            .center_y(iced::Length::Fill)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into();

        if self.debug {
            content.explain(Color::BLACK)
        } else {
            content
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Home,
    Settings,
    Book
}

impl Screen {

}

impl Default for Library{
    fn default() -> Self {
        Self {
            screen: Screen::Home,
            debug: false,
        }
    }
}
