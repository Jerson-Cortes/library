use iced::{
    Color, Element,
};

use iced::{
    widget,
    window::Settings as WindowSettings
};

mod views;

use views::{
    home::home_screen,
    settings::settings_screen,
};

pub fn main() -> iced::Result {
    iced::application(Library::default, Library::update, Library::view)
        .title(Library::title)
        .centered()
        .run()
}

pub struct Library {
    screen: Screen,
    debug_toggle: bool,
    window_settings: WindowSettings,
}

#[derive(Debug, Clone)]
pub enum Message {
    DebugToggle,
    DecorationsToggle(bool)
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
                self.debug_toggle = !self.debug_toggle
            },
            Message::DecorationsToggle(decoration_toggler) => {
                // iced::window::toggle_decorations();
            }
        }
    }

    fn view(&self)  -> Element<'_, Message> {

        let screen = match self.screen {
            Screen::Home => home_screen(),
            Screen::Settings => settings_screen(self),
            Screen::Book => home_screen(),
        };

        let content: Element<_> = widget::container(screen)
            .center_x(iced::Length::Fill)
            .center_y(iced::Length::Fill)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into();

        if self.debug_toggle {
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
            screen: Screen::Settings,
            debug_toggle: false,
            window_settings: WindowSettings {
                ..WindowSettings::default()
            },
        }
    }
}
