use iced::widget;
use iced::{Element, Settings, Task, widget::column};

mod screen;
mod window;

use crate::screen::home::home_screen;

use self::screen::Screen;
use self::window::LibraryWindow;

pub fn main() -> iced::Result {
    iced::daemon(Library::new, Library::update, Library::view)
        .title(Library::title)
        .run()
}

pub struct Library {
    screen: Screen,
    settings: Settings,
    main_window: LibraryWindow,
}

#[derive(Debug, Clone)]
pub enum Message {
    ShowWindow,
}

impl Library {
    fn new() -> (Self, Task<Message>) {
        let (main_window_id, open_main_window) = window::open(window::Settings::default());
        let main_window = LibraryWindow::new(main_window_id);
        let commands = vec![open_main_window.then(|_| Task::none())];
        (
            Self {
                screen: Screen::Home,
                settings: Settings::default(),
                main_window,
            },
            Task::batch(commands),
        )
    }

    fn title(&self, _window_id: window::Id) -> String {
        let screen = match self.screen {
            Screen::Home => "Home",
        };

        format!("{screen} - Library")
    }

    fn update(&mut self, event: Message) -> Task<Message> {
        match event {
            _ => Task::none(),
        }
    }

    fn view(&self, id: window::Id) -> Element<'_, Message> {
        if self.main_window.id == id {
            let screen = match self.screen {
                Screen::Home => home_screen(),
            };

            let content: Element<_> = widget::container(screen)
                .center_x(iced::Length::Fill)
                .center_y(iced::Length::Fill)
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .into();

            content
        } else {
            column![].into()
        }
    }
}
