use iced::{
    Element, Settings, Task,
    widget::column,
    window::{self, Settings as WindowSettings},
};

pub fn main() -> iced::Result {
    iced::daemon(Library::new, Library::update, Library::view)
        .title(Library::title)
        .run()
}

pub struct Library {
    screen: Screen,
    debug_toggle: bool,
    settings: Settings,
    window_settings: WindowSettings,
}

#[derive(Debug, Clone)]
pub enum Message {
    ShowWindow,
}

impl Library {
    fn new() -> (Self, Task<Message>) {
        (Library::default(), Task::none())
    }

    fn title(&self, _window_id: window::Id) -> String {
        let screen = match self.screen {
            Screen::Welcome => "Welcome",
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
        column![].into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Welcome,
    Home,
}

impl Screen {}

impl Default for Library {
    fn default() -> Self {
        Self {
            screen: Screen::Welcome,
            debug_toggle: false,
            settings: Settings {
                ..Settings::default()
            },
            window_settings: WindowSettings {
                decorations: false,
                ..WindowSettings::default()
            },
        }
    }
}
