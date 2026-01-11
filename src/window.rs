pub use iced::window::{Id, Settings, open};

#[derive(Debug)]
pub struct LibraryWindow {
    pub id: Id,
    pub is_visible: bool,
}

impl LibraryWindow {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            is_visible: false,
        }
    }
}
